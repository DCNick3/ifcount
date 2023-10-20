![test suite](https://github.com/DCNick3/ifcount/actions/workflows/test.yml/badge.svg?event=push)

![](media/ifcount.gif)

A _long awaited_ solution for a _widely encountered_ problem!

The <img src="media/ifcount.gif" alt="ifcount" height="18" style="vertical-align: middle;"/> will count the number of ifs in your rust project!

(it can also collect some other numerical metrics, but we know what you are here for)

## Usage example

To run against a github repo:

```bash
$ ifcount collect-github-repo DCNick3/shin
```

This command will run all the metric collectors and print the results to stdout in json format.

The fetched sources will be cached in `~/.cache/ifcount` (see [docs for directories crate](https://docs.rs/directories/latest/directories/) for locations on other OSes) so that the next time you run the command it will be much faster.

If you want to run against a local repo, you can do so. NOTE: this will not collect some metrics that are specific to github repos (like number of stars, number of forks, etc.)

The command is:

```bash
$ ifcount collect-local-repo path_to_your_repo
```

## Json example

```json
{
  "meta": {
    "url": "git@github.com:DCNick3/shin.git"
  },
  "metrics": {
    "avg_fn_depth": 1.2714041095890414,
    "if_count": 429
  }
}
```
