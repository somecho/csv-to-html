# csv-to-html

A simple CLI and Rust crate to convert CSV to HTML tables.

## Installation
Installation is very easy with Cargo. If you have it:

```sh
cargo install csv-to-html
```

### Uninstalling
`csv-to-html` has no side effects on your system, _i.e. no config files, no
program data files, etc_. Installation is as easy as:

```sh
cargo uninstall csv-to-html
```

### Web Service

`csv-to-html` has a [web
service](https://github.com/somecho/csv-to-html-service) that wraps this tool.
If you just need a quick and easy conversion, try it out.

## Usage

```
Usage: csv-to-html [OPTIONS] --filename <FILENAME>

Options:
  -f, --filename <FILENAME>    path to input file
  -n, --no-header              the CSV file has no header
  -o, --output <OUTPUT>        path of output file. If not specified, will print to stdout
  -d, --delimiter <DELIMITER>  [default: ,]
  -h, --help                   Print help
  -V, --version                Print version
```

### Examples
```sh
csv-to-html -f simple.csv
csv-to-html -f simple.csv -n
csv-to-html -f simple.csv -d " "
csv-to-html -f simple.csv -o output.html
```

## Programmatic Usage
You can also use this is a library. To add to your Rust program:
```
cargo add csv-to-html
```

### Example

```rust
let html: String = csv_to_html::convert(&csv_string, &b',', &true);
```
