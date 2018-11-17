
extern crate walkdir;
extern crate log;

use std::cmp::Ordering;
use std::path::Path;
use std::io::ErrorKind;

use walkdir::{DirEntry, WalkDir};
use log::{warn};

/// Specifies the sorting algorithm by filesize.
pub enum SortBy {
    /// Sorts by filesize ascending
    Ascending,
    /// Sorts by filesize descending
    Descending,
}

/// Options when constructing an `LRG` struct.
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
    /// The default function
    ///
    /// For example:
    /// ```
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

/// The main struct for searching for files by size
#[derive(Clone, Debug)]
pub struct Lrg {
    entries: Vec<DirEntry>,
}

// TODO tests
// TODO test calling on a file
// TODO test following links

impl Lrg {
    /// Creates a new Lrg with options and at the given path
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
                    match options.include_dirs {
                        true => entries.push(entry.to_owned()),
                        false => { 
                            if entry.file_type().is_file() { 
                                entries.push(entry.to_owned());
                            }
                        },
                    }
                },
                Err(err) => {
                    let path = err.path().unwrap_or_else(|| Path::new("")).display();
                    let error_message = get_walkdir_error_str(&err);
                    println!("lrg: error opening '{}': {}", path, error_message);
                },
            }
        }

        Lrg {
            entries,
        }
    }

    /// Sorts the lrg object entries, and returns the lrg object
    pub fn sort_by(self, cmp: &SortBy) -> Self {
        match cmp {
            SortBy::Ascending => self.sort_ascending(),
            SortBy::Descending => self.sort_descending(),
        }
    }

    /// Sorts the lrg object entries by a custom sort function, and returns the lrg object
    pub fn sort_by_custom<F>(mut self, cmp: F) -> Self 
    where F: FnMut(&DirEntry, &DirEntry) -> Ordering
    {
        self.entries.sort_unstable_by(cmp);
        self
    }

    // Sorts the lrg object entries by ascending file size, and returns the lrg object
    pub fn sort_ascending(mut self) -> Self {
        self.entries.sort_unstable_by(|a: &DirEntry, b: &DirEntry| {
            Self::get_size(a).cmp(&Self::get_size(b))    
        });
        self
    }

    // Sorts the lrg object entries by descending file size, and returns the lrg object
    pub fn sort_descending(mut self) -> Self {
        self.entries.sort_unstable_by(|a: &DirEntry, b: &DirEntry| {
            Self::get_size(b).cmp(&Self::get_size(a))    
        });
        self
    }

    fn get_size(entry: &DirEntry) -> u64 {
        match entry.metadata() {
            Ok(meta) => meta.len(),
            Err(err) => {
                warn!("Couldn't get metadata for {}: {:?}", entry.path().display(), err);
                0
            },
        }
    }

    /// Gets the entries from the lrg object
    pub fn get_entries(&self) -> Vec<DirEntry> {
        self.entries.clone()
    }
}

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
        },
        None => "Unknown error".to_owned(),
    }
}


