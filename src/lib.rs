
use std::cmp::Ordering;
use std::path::{Path};

use walkdir::{DirEntry, WalkDir};

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

// TODO error handling logs
// TODO don't recurse Lrg::new(path, OPTIONS).sort_by().get_entries()

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
                Ok(entry) => if entry.file_type().is_file() { 
                    entries.push(entry.to_owned())
                },
                Err(error) => println!("Error viewing file"),
            }
        }

        Lrg {
            entries,
        }
    }

    pub fn sort_by(self, cmp: SortBy) -> Self {
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
            Err(err) => 0,
        }
    }

    pub fn get_entries(&self) -> Vec<DirEntry> {
        self.entries.clone()
    }
}

// TODO tests
// TODO test calling on a file

