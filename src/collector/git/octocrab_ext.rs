use anyhow::{Context, Result};
use async_trait::async_trait;
use governor::clock::DefaultClock;
use governor::state::{InMemoryState, NotKeyed};
use governor::{Quota, RateLimiter};
use http::Response;
use hyper::Body;
use octocrab::models::commits::Commit;
use octocrab::models::{Rate, Repository};
use octocrab::{FromResponse, Octocrab, Page};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use snafu::GenerateImplicitData;
use std::num::NonZeroU32;
use std::path::PathBuf;
use tracing::{instrument, Instrument};
use url::Url;

type DefaultRateLimiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;

async fn make_rate_limiter(rate: Rate) -> Result<DefaultRateLimiter> {
    // assuming all rates are reset every minute
    let quota = Quota::per_minute(NonZeroU32::new(rate.limit as u32).unwrap());
    let rate_limiter = RateLimiter::direct(quota);
    // preload
    if let Some(used) = NonZeroU32::new(rate.remaining as u32) {
        rate_limiter.until_n_ready(used).await.unwrap();
    }
    Ok(rate_limiter)
}

struct Cache {
    directory: PathBuf,
}

impl Cache {
    pub fn new(directory: PathBuf) -> Self {
        Self { directory }
    }

    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        let Some(meta) = cacache::index::find_async(&self.directory, key)
            .await
            .context("Finding cache entry")?
        else {
            return Ok(None);
        };
        let content = cacache::read_hash(&self.directory, &meta.integrity)
            .await
            .context("Reading cache entry")?;
        let content = serde_json::from_slice(&content).context("Deserializing cache entry")?;
        Ok(Some(content))
    }

    pub async fn set<T: Serialize>(&self, key: &str, value: &T) -> Result<()> {
        let content = serde_json::to_vec(value).context("Serializing cache entry")?;
        cacache::write(&self.directory, key, &content)
            .await
            .context("Writing cache entry")?;
        Ok(())
    }
}

struct RawBody(String);

#[async_trait]
impl FromResponse for RawBody {
    async fn from_response(response: Response<Body>) -> octocrab::Result<Self> {
        let (_, body) = response.into_parts();
        let body = hyper::body::to_bytes(body)
            .await
            .map_err(|source| octocrab::Error::Hyper {
                source,
                backtrace: snafu::Backtrace::generate(),
            })?;

        let body =
            String::from_utf8(body.to_vec()).map_err(|source| octocrab::Error::InvalidUtf8 {
                source,
                backtrace: snafu::Backtrace::generate(),
            })?;

        Ok(Self(body))
    }
}

pub struct LimitedCrab {
    crab: Octocrab,
    cache: Cache,
    api_rate_limiter: DefaultRateLimiter,
    user_content_rate_limiter: DefaultRateLimiter,
}

impl LimitedCrab {
    pub async fn new(token: Option<String>, cache_dir: PathBuf) -> Result<Self> {
        let mut crab_builder = Octocrab::builder();
        if let Some(token) = token {
            crab_builder = crab_builder.personal_token(token);
        }
        let crab = crab_builder.build().context("Building octocrab")?;

        let rate_limit = crab
            .ratelimit()
            .get()
            .await
            .context("Getting the rate limit")?;

        let api_rate_limiter = make_rate_limiter(rate_limit.resources.core).await?;
        let user_content_rate_limiter =
            RateLimiter::direct(Quota::per_hour(NonZeroU32::new(5000).unwrap()));

        let cache = Cache::new(cache_dir.clone());

        Ok(Self {
            crab,
            cache,
            api_rate_limiter,
            user_content_rate_limiter,
        })
    }

    #[instrument(skip(self))]
    pub async fn get_commit_count(&self, repo_name: &str) -> Result<u32> {
        self.api_rate_limiter
            .until_ready()
            .instrument(tracing::info_span!("wait_rate_limit"))
            .await;

        let page: Page<Commit> = self
            .crab
            .get(
                format!("/repos/{repo_name}/commits?per_page=1",),
                None::<&()>,
            )
            .await
            .context("Getting commits")?;

        let last_page_url = Url::parse(&page.last.unwrap().to_string()).unwrap();
        let (_, commits) = last_page_url
            .query_pairs()
            .find(|(k, _)| k == "page")
            .unwrap();
        let commits = commits.parse::<u32>().unwrap();

        Ok(commits)
    }

    #[instrument(skip(self))]
    pub async fn get_repo_tree(&self, repo_name: &str) -> Result<(String, Tree)> {
        self.api_rate_limiter
            .until_ready()
            .instrument(tracing::info_span!("wait_rate_limit"))
            .await;

        let page: Page<Commit> = self
            .crab
            .get(
                format!("/repos/{repo_name}/commits?per_page=1",),
                None::<&()>,
            )
            .await
            .context("Getting commits")?;

        let latest_commit_sha = &page.items[0].sha;

        let cache_key = format!("tree/{}", latest_commit_sha);

        if let Some(cached) = self
            .cache
            .get(&cache_key)
            .await
            .context("Reading from cache")?
        {
            return Ok((latest_commit_sha.to_string(), cached));
        }

        self.api_rate_limiter
            .until_ready()
            .instrument(tracing::info_span!("wait_rate_limit"))
            .await;
        let tree: RateLimitInfo<Tree> = self
            .crab
            .get(
                format!("/repos/{repo_name}/git/trees/{latest_commit_sha}?recursive=true"),
                None::<&()>,
            )
            .instrument(tracing::info_span!("get_tree"))
            .await
            .context("Getting the tree")?;
        if let Some(used) = NonZeroU32::new(tree.used - 1) {
            self.api_rate_limiter
                .until_n_ready(used)
                .instrument(tracing::info_span!("wait_rate_limit"))
                .await
                .context("Waiting for rate limit")?;
        }

        let tree = tree.content;
        self.cache
            .set(&cache_key, &tree)
            .await
            .context("Writing to cache")?;

        Ok((latest_commit_sha.to_string(), tree))
    }

    #[instrument(skip(self))]
    pub async fn get_file(&self, repo_name: &str, commit: &str, path: &str) -> Result<String> {
        self.user_content_rate_limiter
            .until_ready()
            .instrument(tracing::info_span!("wait_rate_limit"))
            .await;

        let cache_key = format!("file/{repo_name}/{commit}/{path}");
        if let Some(cached) = self
            .cache
            .get(&cache_key)
            .await
            .context("Reading from cache")?
        {
            return Ok(cached);
        }

        let url = format!("https://raw.githubusercontent.com/{repo_name}/{commit}/{path}");

        let RawBody(contents) = self
            .crab
            .get(url, None::<&()>)
            .await
            .context("Getting the file")?;

        self.cache
            .set(&cache_key, &contents)
            .await
            .context("Writing to cache")?;

        Ok(contents)
    }

    #[instrument(skip(self))]
    pub async fn get_repo_info(&self, repo_name: &str) -> Result<Repository> {
        let (owner, repo) = repo_name.split_once('/').unwrap();

        self.user_content_rate_limiter
            .until_ready()
            .instrument(tracing::info_span!("wait_rate_limit"))
            .await;
        let info = self
            .crab
            .repos(owner, repo)
            .get()
            .await
            .context("Getting repo info")?;

        Ok(info)
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum TreeItemType {
    Tree,
    Blob,
    Commit,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct TreeItem {
    pub path: String,
    pub mode: String,
    #[serde(rename = "type")]
    pub type_: TreeItemType,
    pub sha: String,
    pub url: Option<Url>,
    pub size: Option<u64>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Tree {
    pub sha: String,
    pub url: Url,
    pub tree: Vec<TreeItem>,
    pub truncated: bool,
}

#[allow(unused)]
struct RateLimitInfo<T: DeserializeOwned> {
    content: T,
    limit: u32,
    used: u32,
    remaining: u32,
    resource: String,
}

#[async_trait::async_trait]
impl<T: DeserializeOwned> FromResponse for RateLimitInfo<T> {
    async fn from_response(response: Response<Body>) -> octocrab::Result<Self> {
        let headers = response.headers();
        let limit = headers
            .get("x-ratelimit-limit")
            .unwrap()
            .to_str()
            .unwrap()
            .parse()
            .unwrap();
        let used = headers
            .get("x-ratelimit-used")
            .unwrap()
            .to_str()
            .unwrap()
            .parse()
            .unwrap();
        let remaining = headers
            .get("x-ratelimit-remaining")
            .unwrap()
            .to_str()
            .unwrap()
            .parse()
            .unwrap();
        let resource = headers
            .get("x-ratelimit-resource")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let json: serde_json::Value = serde_json::from_slice(
            hyper::body::to_bytes(response.into_body())
                .await
                .map_err(|source| octocrab::Error::Hyper {
                    source,
                    backtrace: snafu::Backtrace::generate(),
                })?
                .as_ref(),
        )
        .map_err(|source| octocrab::Error::Serde {
            source,
            backtrace: snafu::Backtrace::generate(),
        })?;

        Ok(Self {
            content: serde_json::from_value(json).map_err(|source| octocrab::Error::Serde {
                source,
                backtrace: snafu::Backtrace::generate(),
            })?,
            limit,
            used,
            remaining,
            resource,
        })
    }
}
