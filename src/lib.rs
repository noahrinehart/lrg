/*! 
`lrg` is a library for find the largest (or smallest) files in a given directory.
There is also support for searching by a custom function, given [`DirEntry`]'s.
In addition to this, you can specify (in [`LrgOptions`]) the minimum depth and maximum depth to search for,
such as if you wanted to prevent recursion. You can also speficy whether to follow links or to include
directories.

Note: [`DirEntry`] is a typedef of [`walkdir::DirEntry`]

## Examples
To find the largest files in a directory:
```
use std::path::Path;
use lrg::{Lrg, LrgOptions, DirEntry, SortBy};
// Get a path to some directory (or file)
let path = Path::new("./some/path");
// Create the Lrg object to store the entries
let mut lrg: Lrg = Lrg::new(path, &LrgOptions::default());
// Sort and get the entries
let mut entries: Vec<DirEntry> = lrg.sort_by(&SortBy::Descending).get_entries();
// You can also call `sort_descending`
entries = lrg.sort_descending().get_entries();
// These calls mutate the underlying struct, so calling:
entries = lrg.get_entries();
// Will give you the same as the call before it
```

To find the smallest files in a directory:
```
# use std::path::Path;
# use lrg::{Lrg, LrgOptions, DirEntry};
let path = Path::new("./some/other/path");
let mut lrg: Lrg = Lrg::new(path, &LrgOptions::default());
let entries: Vec<DirEntry> = lrg.sort_ascending().get_entries();
```

To search using a custom function:
```
# use std::path::Path;
# use lrg::{Lrg, LrgOptions, DirEntry};
let path = Path::new("./another/path");
let mut lrg: Lrg = Lrg::new(path, &LrgOptions::default());
// Sort by filename (note: not the full path)
lrg.sort_by_custom(|a: &DirEntry, b: &DirEntry| {
    a.file_name().cmp(b.file_name())
});
let entries: Vec<DirEntry> = lrg.get_entries();
```

[`Lrg`]: struct.Lrg.html
[`DirEntry`]: struct.DirEntry.html
[`LrgOptions`]: struct.LrgOptions.html
[`walkdir::DirEntry`]: https://docs.rs/walkdir/2.2.7/walkdir/struct.DirEntry.html
*/

use std::cmp::Ordering;
use std::io::ErrorKind;
use std::path::Path;

use log::warn;
use walkdir::WalkDir;

/// Specifies the sorting algorithm.
pub enum SortBy {
    /// Sorts by filesize ascending
    Ascending,
    /// Sorts by filesize descending
    Descending,
}

/// Options when constructing an `Lrg` struct.
///
/// # Examples
/// Can be constructed like normal:
/// ```
/// # use lrg::LrgOptions;
/// let opts = LrgOptions {
///     min_depth: 1,
///     max_depth: 5,
///     follow_links: false,
///     include_dirs: true,
/// };
/// ```
/// Or can also inherit [`default options`]:
/// ```
/// # use lrg::LrgOptions;
/// let opts = LrgOptions {
///     min_depth: 5,
///     max_depth: 10,
///     ..LrgOptions::default()
/// };
/// ```
///
/// [`default options`]: struct.LrgOptions.html#method.default
#[derive(Clone, Debug)]
pub struct LrgOptions {
    /// Specifies them minimum depth for searching
    /// Minimum depth is the depth at which to start searching
    pub min_depth: usize,
    // Specifies the maximum depth for searching
    // Maximum depth is the depth at which to stop searching
    pub max_depth: usize,
    /// Specifies whether to follow links while searching
    pub follow_links: bool,
    /// Speicifies whether to include directories in the search
    pub include_dirs: bool,
}

/// Implements default options
impl Default for LrgOptions {
    /// The default function.
    ///
    /// # Examples
    /// ```
    /// use lrg::LrgOptions;
    /// // Gives options that recurse as far as possible, don't follow links,
    /// // and don't include directories.
    /// let options = LrgOptions::default();
    /// ```
    fn default() -> LrgOptions {
        LrgOptions {
            min_depth: 0,
            max_depth: std::usize::MAX,
            follow_links: false,
            include_dirs: false,
        }
    }
}

/// A type copy of the [`walkdir::DirEntry`] struct.
///
/// [`walkdir::DirEntry`]: https://docs.rs/walkdir/2.2.7/walkdir/struct.DirEntry.html
pub type DirEntry = walkdir::DirEntry;

/// The main struct for searching for files by size.
/// Constructed using [`new`], passing in a path and options.
///
/// [`new`]: struct.Lrg.html#method.new
#[derive(Clone, Debug)]
pub struct Lrg {
    entries: Vec<DirEntry>,
}

impl Lrg {
    /// Creates a new Lrg with options and at the given path.
    ///
    /// # Examples
    /// ```
    /// # use std::path::Path;
    /// # use lrg::{Lrg, LrgOptions};
    /// let path = Path::new(".");
    /// let lrg = Lrg::new(path, &LrgOptions::default());
    /// ```
    /// To use custom options, just supply a [`LrgOptions`] struct.
    ///
    /// To only search the base directoy, using the other default options:
    /// ```
    /// # use std::path::Path;
    /// # use lrg::LrgOptions;
    /// let path = Path::new(".");
    /// let opts = LrgOptions {
    ///     min_depth: 1,
    ///     ..LrgOptions::default()
    /// };
    /// ```
    ///
    /// [`LrgOptions`]: struct.LrgOptions.html
    pub fn new(path: &Path, options: &LrgOptions) -> Self {
        let mut entries: Vec<DirEntry> = Vec::new();

        // Walk directory recursivley (prints debug messages if error)
        for entry in WalkDir::new(&path)
            .min_depth(options.min_depth)
            .max_depth(options.max_depth)
            .follow_links(options.follow_links)
        {
            match entry {
                // Entry can be found
                Ok(entry) => {
                    if entry.file_type().is_dir() && options.include_dirs {
                        entries.push(entry.to_owned())
                    } else if entry.file_type().is_file() || entry.file_type().is_symlink() {
                        entries.push(entry.to_owned());
                    }
                }
                Err(err) => {
                    let path = err.path().unwrap_or_else(|| Path::new("")).display();
                    let error_message = get_walkdir_error_str(&err);
                    println!("lrg: error opening '{}': {}", path, error_message);
                }
            }
        }

        Lrg { entries }
    }

    /// Sorts the lrg object entries, and returns the lrg object.
    ///
    /// # Examples
    /// To get the largest files first:
    /// ```
    /// # use std::path::Path;
    /// # use lrg::{Lrg, LrgOptions, SortBy};
    /// let path = Path::new(".");
    /// let mut lrg = Lrg::new(path, &LrgOptions::default());
    /// lrg.sort_by(&SortBy::Descending);
    /// ```
    pub fn sort_by(&mut self, cmp: &SortBy) -> &Self {
        match cmp {
            SortBy::Ascending => self.sort_ascending(),
            SortBy::Descending => self.sort_descending(),
        }
    }

    /// Sorts the lrg object entries by a custom sort function, and returns the lrg object.
    ///
    /// # Examples
    /// To search by creation date:
    /// ```
    /// # use std::path::{Path, PathBuf};
    /// # use std::ffi::OsStr;
    /// # use lrg::{Lrg, LrgOptions, DirEntry};
    /// let path = Path::new("./another/path");
    /// let mut lrg = Lrg::new(path, &LrgOptions::default());
    /// lrg.sort_by_custom(|a: &DirEntry, b: &DirEntry| {
    ///     // Create custom function to get creation date of a `DirEntry`
    ///     let creation_date = |x: &DirEntry| {
    ///         match x.metadata() {
    ///             Ok(meta) => {
    ///                 match meta.created() {
    ///                     Ok(created) => created,
    ///                     Err(_) => std::time::SystemTime::UNIX_EPOCH,
    ///                 }
    ///             },
    ///             // Default to UNIX epoch
    ///             Err(_) => std::time::SystemTime::UNIX_EPOCH,
    ///         }
    ///     };
    ///     //Make comparison
    ///     creation_date(a).cmp(&creation_date(b))
    /// });
    /// // Get entries
    /// let entries: Vec<DirEntry> = lrg.get_entries();
    /// ```
    pub fn sort_by_custom<F>(&mut self, cmp: F) -> &Self
    where
        F: FnMut(&DirEntry, &DirEntry) -> Ordering,
    {
        self.entries.sort_unstable_by(cmp);
        self
    }

    /// Sorts the lrg object entries by ascending file size, and returns the lrg object.
    ///
    /// # Examples
    /// ```
    /// # use std::path::Path;
    /// # use lrg::{Lrg, LrgOptions};
    /// let path = Path::new("./another/path");
    /// let mut lrg: Lrg = Lrg::new(path, &LrgOptions::default());
    /// let entries = lrg.sort_ascending().get_entries();
    /// ```
    pub fn sort_ascending(&mut self) -> &Self {
        self.entries.sort_unstable_by(|a: &DirEntry, b: &DirEntry| {
            Self::get_size(a).cmp(&Self::get_size(b))
        });
        self
    }

    /// Sorts the lrg object entries by descending file size, and returns the lrg object.
    ///
    /// # Examples
    /// ```
    /// # use std::path::Path;
    /// # use lrg::{Lrg, LrgOptions};
    /// let path = Path::new("./another/path");
    /// let mut lrg: Lrg = Lrg::new(path, &LrgOptions::default());
    /// let entries = lrg.sort_descending().get_entries();
    /// ```
    pub fn sort_descending(&mut self) -> &Self {
        self.entries.sort_unstable_by(|a: &DirEntry, b: &DirEntry| {
            Self::get_size(b).cmp(&Self::get_size(a))
        });
        self
    }

    fn get_size(entry: &DirEntry) -> u64 {
        match entry.metadata() {
            Ok(meta) => meta.len(),
            Err(err) => {
                warn!(
                    "Couldn't get metadata for {}: {:?}",
                    entry.path().display(),
                    err
                );
                0
            }
        }
    }

    /// Gets the entries from the [`Lrg`] object.
    ///
    /// # Examples
    /// ```
    /// # use std::path::Path;
    /// # use lrg::{Lrg, LrgOptions};
    /// let path = Path::new("./another/path");
    /// let mut lrg: Lrg = Lrg::new(path, &LrgOptions::default());
    /// let entries = lrg.sort_ascending().get_entries();
    /// ```
    /// [`Lrg`]: struct.Lrg.html
    pub fn get_entries(&self) -> Vec<DirEntry> {
        self.entries.clone()
    }
}

/// This function gets a string for a walkdir error.
/// This is needed since `io_error.to_str()` is not public.
pub fn get_walkdir_error_str(err: &walkdir::Error) -> String {
    match err.io_error() {
        Some(ioerr) => {
            // Because ErrorKind.as_str() is private, if someone finds a fix, send a pr
            match ioerr.kind() {
                ErrorKind::NotFound => "Entity not found".to_owned(),
                ErrorKind::PermissionDenied => "Permission denied".to_owned(),
                ErrorKind::ConnectionRefused => "Connection refused".to_owned(),
                ErrorKind::ConnectionReset => "Connection reset".to_owned(),
                ErrorKind::ConnectionAborted => "Connection aborted".to_owned(),
                ErrorKind::NotConnected => "Not connected".to_owned(),
                ErrorKind::AddrInUse => "Address in use".to_owned(),
                ErrorKind::AddrNotAvailable => "Address not available".to_owned(),
                ErrorKind::BrokenPipe => "Broken pipe".to_owned(),
                ErrorKind::AlreadyExists => "Entity already exists".to_owned(),
                ErrorKind::WouldBlock => "Operation would block".to_owned(),
                ErrorKind::InvalidInput => "Invalid input parameter".to_owned(),
                ErrorKind::InvalidData => "Invalid data".to_owned(),
                ErrorKind::TimedOut => "Timed out".to_owned(),
                ErrorKind::WriteZero => "Write zero".to_owned(),
                ErrorKind::Interrupted => "Operation interrupted".to_owned(),
                ErrorKind::Other => "Other os error".to_owned(),
                ErrorKind::UnexpectedEof => "Unexpected end of file".to_owned(),
                _ => "Unknown error".to_owned(),
            }
        }
        None => "Unknown error".to_owned(),
    }
}

// Tests are located in <PROJECT_ROOT>/tests folder
