# scgld

A filter for command line tools

```shell
Usage scgld [OPTIONS] --target <TARGET> --keyword <KEYWORD> [TARGET_ARGS]...

Arguments:
[TARGET_ARGS]...  Target arguments

Options:
--target <TARGET>    Target binary name
--keyword <KEYWORD>  Keyword of filter
--status             If set, scgld will exit with target's status code
-h, --help               Print help
-V, --version            Print version
```

## Example

The original purpose of this tool is to add output filter for staticcheck in Goland...(StaticCheck GoLanD -> scgld XD)

So let's stop Goland from listing all warnings of your project.

Add a File Watcher after installation of staticcheck:

<img width="649" alt="image" src="https://github.com/miranquil/scgld/assets/25676311/8b90ac71-da53-4d9a-99e1-9168c88ab15a">

Then the output of staticcheck will be filtered to current file only.

