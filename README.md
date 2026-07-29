# grepme

A small `grep`-like command-line tool written in Rust, built as part of working through
[the Rust Book's grepme project](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
and extended with extra features such as full regex support (via `fancy-regex`), automatic
directory recursion, colored output, and writing results to a file.

## Features

- **Regex search** - queries are compiled as regular expressions (including lookaround,
  backreferences, etc. thanks to [`fancy-regex`](https://crates.io/crates/fancy-regex)),
  so you can pass either a plain string or a full pattern ([test your syntax](https://fancy-regex.github.io/fancy-regex/)).
- **Recursive directory search** - pass a folder as a target and every file inside it
  (recursively) is searched automatically.
- **Multiple targets** - search any number of files and/or folders in a single invocation.
- **Case-insensitive search** - via a flag or the `IGNORE_CASE` environment variable.
- **Colored output** - matches are highlighted in the terminal using
  [`colored`](https://crates.io/crates/colored). Color is automatically disabled when
  writing output to a file.
- **Symlinks are skipped** (not followed), and I/O errors are reported as non-blocking
  warnings so the rest of the search can continue.
- **Output to file** - redirect results to a file instead of stdout, ignores ouput file in search.

## Installation

Clone the repository and build with Cargo:

```sh
git clone <this-repo-url>
cd grepme
cargo build --release
```

The compiled binary will be available at `target/release/grepme`.

## Usage

```
grepme [arguments] query [paths/glob]
```

- `query` - a plain string or a quoted regex pattern to search for.
- `paths/glob` - one or more files or directories to search. Directories are searched
  recursively.

### Arguments

| Flag                             | Description                               |
| -------------------------------- | ----------------------------------------- |
| `-h`, `--help`                   | Show the help text                        |
| `-i`, `--ignore-case`            | Perform a case-insensitive search         |
| `-o`, `--output-file <filepath>` | Write results to a file instead of stdout |

### Environment variables

- `IGNORE_CASE` - if set to `true`, enables case-insensitive search by default. This can
  still be overridden by explicitly passing `-i`/`--ignore-case`.

### Examples

Search for the literal string `TODO` in a single file:

```sh
grepme duck src/main.rs
```

Recursively search an entire directory, case-insensitively:

```sh
grepme -i "duck" src/
```

Search with a regex pattern and write the results to a file:

```sh
grepme -o results.txt "(?i)duck" src/
```

Search multiple files/folders in one command:

```sh
grepme "duck" src/ Cargo.toml
```

## Project structure

```
src/
  main.rs      # CLI entry point: argument handling, running the search, printing output
  config.rs    # Config parsing (Config::build) and the help text
  search.rs    # Core search logic (search_valid_files)
  targets.rs   # Expands file/folder arguments into a flat list of files to search
```

- `Config::build` (in `config.rs`) parses the raw CLI arguments into a `Config` struct,
  handling flags, the query, and target paths.
- `targets::expand_to_files` (in `targets.rs`) walks the given paths, recursing into
  directories, skipping symlinks, and avoiding re-reading the output file if one is
  specified.
- `search::search_valid_files` (in `search.rs`) runs the compiled regex against each line of a
  file's contents and returns the matching line numbers and text.
- `main.rs` wires these pieces together: it builds the config, expands targets, compiles
  the regex, searches each file, and prints or writes the formatted results.

## Running tests

```sh
cargo test
```

## License

Licensed under either of the [MIT license](LICENSE-MIT) or the
[Apache License, Version 2.0](LICENSE-APACHE), at your option.
