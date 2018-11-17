# Lrg
> A utility to help find the largest file(s) in a directory

## Examples

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