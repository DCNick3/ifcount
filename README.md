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

```json5
{
  "meta": {
    "url": "git@github.com:DCNick3/ifcount.git",
    "commit": "796ae0c921825b812fd0f25bdda2dd42005806c5"
  },
  "metrics": {
    "metric1": [1,2,3,4,5],
    "metric2": [1,2,3],
    // ...
  }
}
```

## Metrics

### File metrics

- `per_file.all_fn_count.avg`: Average number of functions per file.
- `per_file.all_fn_count.mode`: Mode (most common) number of functions per file.
- `per_file.all_fn_count.sum`: Total number of functions across all files.
- `per_file.enum_count.avg`: Average number of enums per file.
- `per_file.enum_count.mode`: Mode (most common) number of enums per file.
- `per_file.enum_count.sum`: Total number of enums across all files.
- `per_file.impl_block_count.avg`: Average number of impl blocks per file.
- `per_file.impl_block_count.mode`: Mode (most common) number of impl blocks per file.
- `per_file.impl_block_count.sum`: Total number of impl blocks across all files.
- `per_file.pub_fn_count.avg`: Average number of public functions per file.
- `per_file.pub_fn_count.mode`: Mode (most common) number of public functions per file.
- `per_file.pub_fn_count.sum`: Total number of public functions across all files.
- `per_file.struct_count.avg`: Average number of structs per file.
- `per_file.struct_count.mode`: Mode (most common) number of structs per file.
- `per_file.struct_count.sum`: Total number of structs across all files.
- `macro.count_per_file.avg`: Average number of macros per file.
- `macro.count_per_file.mode`: Mode of the number of macros per file.
- `macro.count_per_file.sum`: Total number of macros per file.
- `rca.cognitive.average`: Average cognitive complexity averaged per file. [1]
- `rca.cognitive.max`: Maximum cognitive complexity averaged per file. [1]
- `rca.cognitive.min`: Minimum cognitive complexity averaged per file. [1]
- `rca.cognitive.sum`: Total cognitive complexity averaged per file. [1]
- `rca.cyclomatic.average`: Average cyclomatic complexity averaged per file. [1]
- `rca.cyclomatic.max`: Maximum cyclomatic complexity averaged per file. [1]
- `rca.cyclomatic.min`: Minimum cyclomatic complexity averaged per file. [1]
- `rca.cyclomatic.sum`: Total cyclomatic complexity averaged per file. [1]
- `rca.halstead.N1`: Halstead metric N1 averaged over files. [1]
- `rca.halstead.N2`: Halstead metric N2 averaged over files. [1]
- `rca.halstead.bugs`: Halstead metric: Estimated number of bugs averaged per file. [1]
- `rca.halstead.difficulty`: Halstead metric: Difficulty of the code averaged per file. [1]
- `rca.halstead.effort`: Halstead metric: Effort required to write the code averaged per file. [1]
- `rca.halstead.estimated_prog_len`: Halstead metric: Estimated program length averaged per file. [1]
- `rca.halstead.length`: Halstead metric: Program length averaged per file. [1]
- `rca.halstead.level`: Halstead metric: Program level averaged per file. [1]
- `rca.halstead.n1`: Halstead metric N1 averaged over files. [1]
- `rca.halstead.n2`: Halstead metric N2 averaged over files. [1]
- `rca.halstead.purity_ratio`: Halstead metric: Purity ratio averaged per file. [1]
- `rca.halstead.time`: Halstead metric: Estimated time to write the code averaged per file. [1]
- `rca.halstead.vocabulary`: Halstead metric: Vocabulary of the code averaged per file. [1]
- `rca.halstead.volume`: Halstead metric: Code volume averaged per file. [1]
- `rca.loc.blank`: Total number of blank lines of code averaged per file. [1]
- `rca.loc.blank_average`: Average number of blank lines of code averaged per file. [1]
- `rca.loc.blank_max`: Maximum number of blank lines of code averaged per file. [1]
- `rca.loc.blank_min`: Minimum number of blank lines of code averaged per file. [1]
- `rca.loc.cloc`: Total number of comment lines of code averaged per file. [1]
- `rca.loc.cloc_average`: Average number of comment lines of code averaged per file. [1]
- `rca.loc.cloc_max`: Maximum number of comment lines of code averaged per file. [1]
- `rca.loc.cloc_min`: Minimum number of comment lines of code averaged per file. [1]
- `rca.loc.lloc`: Total number of logical lines of code averaged per file. [1]
- `rca.loc.lloc_average`: Average number of logical lines of code averaged per file.[1]
- `rca.loc.lloc_max`: Maximum number of logical lines of code averaged per file. [1]
- `rca.loc.lloc_min`: Minimum number of logical lines of code averaged per file. [1]
- `rca.loc.ploc`: Total number of physical lines of code averaged per file. [1]
- `rca.loc.ploc_average`: Average number of physical lines of code averaged per file. [1]
- `rca.loc.ploc_max`: Maximum number of physical lines of code averaged per file. [1]
- `rca.loc.ploc_min`: Minimum number of physical lines of code averaged per file. [1]
- `rca.loc.sloc`: Total number of source lines of code averaged per file. [1]
- `rca.loc.sloc_average`: Average number of source lines of code averaged per file. [1]
- `rca.loc.sloc_max`: Maximum number of source lines of code averaged per file. [1]
- `rca.loc.sloc_min`: Minimum number of source lines of code averaged per file. [1]
- `rca.mi.mi_original`: Maintainability index (MI) - Original formula averaged per file. [1]
- `rca.mi.mi_sei`: Maintainability index (MI) - SEI formula averaged per file. [1]
- `rca.mi.mi_visual_studio`: Maintainability index (MI) - Visual Studio formula averaged per file. [1]
- `rca.nargs.average`: Average number of arguments in functions averaged per file. [1]
- `rca.nargs.average_closures`: Average number of arguments in closures averaged per file. [1]
- `rca.nargs.average_functions`: Average number of arguments in functions averaged per file. [1]
- `rca.nargs.closures_max`: Maximum number of arguments in closures averaged per file. [1]
- `rca.nargs.closures_min`: Minimum number of arguments in closures averaged per file. [1]
- `rca.nargs.functions_max`: Maximum number of arguments in functions averaged per file. [1]
- `rca.nargs.functions_min`: Minimum number of arguments in functions averaged per file. [1]
- `rca.nargs.total`: Total number of arguments averaged per file. [1]
- `rca.nargs.total_closures`: Total number of arguments in closures averaged per file. [1]
- `rca.nargs.total_functions`: Total number of arguments in functions averaged per file. [1]
- `rca.nexits.average`: Average number of exits (return statements) in functions averaged per file. [1]
- `rca.nexits.max`: Maximum number of exits in functions averaged per file. [1]
- `rca.nexits.min`: Minimum number of exits in functions averaged per file. [1]
- `rca.nexits.sum`: Total number of exits in functions averaged per file. [1]
- `rca.nom.average`: Average number of methods (operations) in classes averaged per file. [1]
- `rca.nom.closures`: Total number of methods in closures averaged per file. [1]
- `rca.nom.closures_average`: Average number of methods in closures averaged per file. [1]
- `rca.nom.closures_max`: Maximum number of methods in closures averaged per file. [1]
- `rca.nom.closures_min`: Minimum number of methods in closures averaged per file. [1]
- `rca.nom.functions`: Total number of methods in functions averaged per file. [1]
- `rca.nom.functions_average`: Average number of methods in functions averaged per file. [1]
- `rca.nom.functions_max`: Maximum number of methods in functions averaged per file. [1]
- `rca.nom.functions_min`: Minimum number of methods in functions averaged per file. [1]
- `rca.nom.total`: Total number of methods (operations) in the codebase averaged per file. [1]

### Function metrics

- `complexity.closure.avg`: Average complexity [2] of closures in the code.
- `complexity.closure.mode`: Mode (most common) complexity [2] of closures in the code.
- `complexity.closure.sum`: Total complexity [2] of closures in the code.
- `complexity.impl_item_fn.avg`: Average complexity [2] of functions within impl blocks.
- `complexity.impl_item_fn.mode`: Mode (most common) complexity [2] of functions within impl blocks.
- `complexity.impl_item_fn.sum`: Total complexity [2] of functions within impl blocks.
- `complexity.item_fn.avg`: Average complexity [2] of standalone functions in the code.
- `complexity.item_fn.mode`: Mode (most common) complexity [2] of standalone functions in the code.
- `complexity.item_fn.sum`: Total complexity [2] of standalone functions in the code.
- `fn_arg_count.avg`: Average number of arguments in functions.
- `fn_arg_count.mode`: Mode (most common) number of arguments in functions.
- `fn_arg_count.sum`: Total number of arguments in functions.
- `fn_depth.avg`: Average depth (nesting) of functions in the code.
- `fn_depth.mode`: Mode (most common) depth (nesting) of functions in the code.
- `fn_depth.sum`: Total depth (nesting) of functions in the code.
- `lcom4_per_impl_block.avg`: Average LCOM4 metric [3] per impl block.
- `lcom4_per_impl_block.mode`: Mode (most common) LCOM4 metric [3] per impl block.
- `lcom4_per_impl_block.sum`: Total LCOM4 metric [3] across all impl blocks.
- `statement_size.avg`: Average size (number of expressions) of statements in the code.
- `statement_size.mode`: Mode (most common) size (number of expressions) of statements in the code.
- `statement_size.sum`: Total size (number of expressions) of statements in the code.
- `macro.argument_size.avg`: Average size of macro arguments.
- `macro.argument_size.mode`: Mode of macro argument size.
- `macro.argument_size.sum`: Total size of macro arguments.

### Definition metrics

- `enums.attr_count.avg`: Average number of attributes per enum.
- `enums.attr_count.mode`: Mode (most common) number of attributes per enum.
- `enums.attr_count.sum`: Total number of attributes across all enums.
- `enums.variant_attr_count.avg`: Average number of attributes per enum variant.
- `enums.variant_attr_count.mode`: Mode (most common) number of attributes per enum variant.
- `enums.variant_attr_count.sum`: Total number of attributes across all enum variants.
- `enums.variant_count.avg`: Average number of variants per enum.
- `enums.variant_count.mode`: Mode (most common) number of variants per enum.
- `enums.variant_count.sum`: Total number of variants across all enums.
- `structs.attrs_count.avg`: Average number of attributes per struct.
- `structs.attrs_count.mode`: Mode (most common) number of attributes per struct.
- `structs.attrs_count.sum`: Total number of attributes across all structs.
- `structs.field_attr_count.avg`: Average number of attributes per struct field.
- `structs.field_attr_count.mode`: Mode (most common) number of attributes per struct field.
- `structs.field_attr_count.sum`: Total number of attributes across all struct fields.
- `structs.fields_count.avg`: Average number of fields per struct.
- `structs.fields_count.mode`: Mode (most common) number of fields per struct.
- `structs.fields_count.sum`: Total number of fields across all structs.
- `structs.public_fields_count.avg`: Average number of public fields per struct.
- `structs.public_fields_count.mode`: Mode (most common) number of public fields per struct.
- `structs.public_fields_count.sum`: Total number of public fields across all structs.
- `trait_def.all_fn_count.avg`: Average number of functions within trait definitions.
- `trait_def.all_fn_count.mode`: Mode (most common) number of functions within trait definitions.
- `trait_def.all_fn_count.sum`: Total number of functions within trait definitions.
- `trait_def.assoc_type_count.avg`: Average number of associated types within trait definitions.
- `trait_def.assoc_type_count.mode`: Mode (most common) number of associated types within trait definitions.
- `trait_def.assoc_type_count.sum`: Total number of associated types within trait definitions.
- `trait_def.default_fn_count.avg`: Average number of default functions within trait definitions.
- `trait_def.default_fn_count.mode`: Mode (most common) number of default functions within trait definitions.
- `trait_def.default_fn_count.sum`: Total number of default functions within trait definitions.
- `trait_def.generic_param_count.avg`: Average number of generic parameters within trait definitions.
- `trait_def.generic_param_count.mode`: Mode (most common) number of generic parameters within trait definitions.
- `trait_def.generic_param_count.sum`: Total number of generic parameters within trait definitions.
- `trait_def.supertrait_count.avg`: Average number of supertraits within trait definitions.
- `trait_def.supertrait_count.mode`: Mode (most common) number of supertraits within trait definitions.
- `trait_def.supertrait_count.sum`: Total number of supertraits within trait definitions.

### Repository metrics

- `repo.commit_count`: Total number of commits in the repository.
- `repo.forks`: Total number of forks for the repository.
- `repo.open_issues`: Total number of open issues for the repository.
- `repo.size`: Size of the repository in kilobytes.
- `repo.stars`: Total number of stars for the repository.
- `repo.watchers`: Total number of watchers for the repository.
- `if_count`: Total number of if statements in the code.


### Metric references

[1] These metrics are collected using the [rust-code-analysis](https://mozilla.github.io/rust-code-analysis/index.html)

[2] Campbell, G. Ann, _Cognitive complexity: an overview and evaluation_, 2018

[3] Allen, E.B. and Khoshgoftaar, T.M., _Measuring coupling and cohesion: an information-theory approach_, 1999