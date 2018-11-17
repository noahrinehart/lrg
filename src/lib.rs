
extern crate walkdir;
extern crate log;

use std::cmp::Ordering;
use std::path::Path;
use std::io::ErrorKind;

use walkdir::{DirEntry, WalkDir};
use log::{warn};

pub enum SortBy {
    Ascending,
    Descending,
}

#[derive(Clone, Debug)]
pub struct Options {
    pub min_depth: usize,
    pub max_depth: usize,
    pub follow_links: bool,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            min_depth: 0,
            max_depth: std::usize::MAX,
            follow_links: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Lrg {
    entries: Vec<DirEntry>,
}

// TODO include dirs
// TODO error handling logs
// TODO tests
// TODO test calling on a file
// TODO test following links

impl Lrg {
    pub fn new(path: &Path, options: &Options) -> Self {
        let mut entries: Vec<DirEntry> = Vec::new();
        
        // Walk directory recursivley (prints debug messages if error)
        for entry in WalkDir::new(&path)
            .min_depth(options.min_depth)
            .max_depth(options.max_depth)
            .follow_links(options.follow_links) 
        {
            match entry {
                Ok(entry) => {
                    if entry.file_type().is_file() { 
                        entries.push(entry.to_owned());
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

    pub fn sort_by(self, cmp: &SortBy) -> Self {
        match cmp {
            SortBy::Ascending => self.sort_ascending(),
            SortBy::Descending => self.sort_descending(),
        }
    }

    pub fn sort_by_custom<F>(mut self, cmp: F) -> Self 
    where F: FnMut(&DirEntry, &DirEntry) -> Ordering
    {
        self.entries.sort_unstable_by(cmp);
        self
    }

    pub fn sort_ascending(mut self) -> Self {
        self.entries.sort_unstable_by(|a: &DirEntry, b: &DirEntry| {
            Self::get_size(a).cmp(&Self::get_size(b))    
        });
        self
    }

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


