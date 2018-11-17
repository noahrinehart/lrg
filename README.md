# lrg
> A utility to help find the largest file(s) in a directory

[![Linux build status](https://api.travis-ci.org/noahrinehart/lrg.svg)](https://travis-ci.org/noahrinehart/lrg)
[![Coverage Status](https://coveralls.io/repos/github/noahrinehart/lrg/badge.svg?branch=master)](https://coveralls.io/github/noahrinehart/lrg?branch=master)
[![crates.io](https://meritbadge.herokuapp.com/lrg)](https://crates.io/crates/lrg)

## Requirements
* [rust](https://www.rust-lang.org/en-US/)

## Building the binary (only tested on macOS and linux)
```sh
# First clone the repo:
git clone git@github.com:noahrinehart/lrg.git
# cd into it:
cd lrg
# Build it:
cargo build --release
# The binary will be in ./target/release/lrg
```

## Examples
### Using the binary

To find the largest files in the current directory (by default, it searches the current directory, and only fetches the top 5):
```sh
./lrg
```

To search another directory (such as the home directory):
```sh
./lrg $HOME
```

To only search in the current directory and not recurse through others:
```sh
./lrg -r
```

#### Full Usage
```
lrg 0.1
Noah Rinehart <rinehart.noah@gmail.com>
A utility to help find the largest file(s) in a directory

USAGE:
    lrg [FLAGS] [OPTIONS] [FILEPATH]

FLAGS:
    -i, --directories     include directories in search (default: false)
    -l, --follow-links    will follow links of files (default: false)
    -r, --no-recursion    will only visit files in specified directory, takes precedence over max-depth (default: false)
    -h, --help            Prints help information
    -V, --version         Prints version information

OPTIONS:
    -d, --max-depth <MAX_DEPTH>    sets the maximum depth of folders to search, unless --no-recursion specified
                                   (default: max possible)
    -n, --number <NUM_ENTRIES>     sets the number of files to list (default: 5)

ARGS:
    <FILEPATH>    the path to search in
```

### Using the library

First, add the crate to your project (check for which version you would like to use, or just put * to use the latest):
```sh
# Cargo.toml
lrg = "VERSION_NUM"
```

Then, add `extern create lrg` at the top of your project.

To find the largest files in a directory:
```rust
use std::path::Path;
use lrg::{Lrg, LrgOptions, DirEntry, SortBy};
// Get a path to some directory (or file)
let path = Path::new("./some/path");
// Create the Lrg object to store the entries
let mut lrg: Lrg = Lrg::new(path, &LrgOptions::default());
// Sort and get the entries
let mut entries: Vec<DirEntry> = lrg.sort_by(SortBy::Descending).get_entries();
// You can also call `sort_descending`
entries = lrg.sort_descending().get_entries();
// These calls mutate the underlying struct, so calling:
entries = lrg.get_entries();
// Will give you the same as the call before it
```

To find the smallest files in a directory:
```rust
let path = Path::new("./some/other/path");
let mut lrg: Lrg = Lrg::new(path, &LrgOptions::default());
let entries: Vec<DirEntry> = lrg.sort_ascending().get_entries();
```

To search using a custom function:
```rust
let path = Path::new("./another/path");
let mut lrg: Lrg = Lrg::new(path, &LrgOptions::default());
// Sort by filename (note: not the full path)
lrg.sort_by_custom(|a: &DirEntry, b: &DirEntry| {
    a.file_name().cmp(b.file_name())
});
let entries: Vec<DirEntry> = lrg.get_entries();
```