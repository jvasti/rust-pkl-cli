```console
$ rust-pkl-cli 
? 2
Usage: rust-pkl-cli <COMMAND>

Commands:
  eval              Render pkl module(s)
  repl              (not implemented) Start a REPL session
  server            (not implemented) Run as a server that communicates over standard input/output
  test              (not implemented) Run tests within the given module(s)
  project           (not implemented) Run commands related to projects
  download-package  (not implemented) Download package(s)
  analyze           (not implemented) Commands related to static analysis
  help              Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

```console
$ rust-pkl-cli --help
Usage: rust-pkl-cli <COMMAND>

Commands:
  eval              Render pkl module(s)
  repl              (not implemented) Start a REPL session
  server            (not implemented) Run as a server that communicates over standard input/output
  test              (not implemented) Run tests within the given module(s)
  project           (not implemented) Run commands related to projects
  download-package  (not implemented) Download package(s)
  analyze           (not implemented) Commands related to static analysis
  help              Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

```console
$ rust-pkl-cli eval --help
Render pkl module(s)

Usage: rust-pkl-cli eval [OPTIONS] <MODULES>

Arguments:
  <MODULES>  Module path to evaluate

Options:
  -f, --format <FORMAT>     Output format to generate [default: json] [possible values: raw, json, yaml, toml]
  -o, --output_path <PATH>  File path where the output file is placed
  -h, --help                Print help

```
