![](media/ifcount.gif)

A _long awaited_ solution for a _widely encountered_ problem!

The <img src="media/ifcount.gif" alt="ifcount" height="18" style="vertical-align: middle;"/> will count the number of ifs in your rust project!

(it can also collect some other numerical metrics, but we know what you are here for)

## Usage example

```bash
$ ifcount collect path_to_your_repo
```

This command will run all the metric collectors and print the results to stdout in json format.

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
