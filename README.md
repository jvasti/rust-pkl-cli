# Rust-pkl-cli

A rust implementation of the [CLI](https://pkl-lang.org/main/current/pkl-cli/index.html) for [PKL language](https://pkl-lang.org/) .

Uses [new-pkl](https://github.com/DevYatsu/new-pkl) to do the actual parsing, so PKL language support is limited to
what it supports.

Only the `eval` subcommand partially supported at this time.

## Usage

```console
$ rust-pkl-cli eval examples/simple.pkl
{
  "job": {
    "company": "Nests R Us",
    "title": "Sr. Nest Maker",
    "yearsOfExperience": 2
  },
  "name": "Swallow"
}

```
