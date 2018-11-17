extern crate lrg;

use std::path::Path;
use std::ffi::OsStr;
use lrg::{Lrg, LrgOptions, DirEntry};

// Creates a vector of strings from strs
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

fn test_entries_against_filenames(entries: &Vec<DirEntry>, filenames: &Vec<String>) {
    assert_eq!(entries.len(), filenames.len());
    for (i, entry) in entries.iter().enumerate() {
        assert_eq!(entry.path().file_name().expect("Cannot get filename"), OsStr::new(&filenames[i]));
    }
}


#[test]
fn test_basic_dir_file_order() {
    // Should count:
    // testdir/
    // ├── subdir/
    // │   ├── subsubdir/
    // │   │   ├── subsubsomefile 2
    // │   ├── link_somefile 7
    // │   ├── subsmallerfile 5
    // │   ├── subsomefile 3
    // ├── evensmallerfile 6
    // ├── smallerfile 4
    // └── somefile 1
    let path = Path::new("tests/testdir");
    let mut lrg = Lrg::new(path, &LrgOptions::default());
    let entries = lrg.sort_descending().get_entries();
    test_entries_against_filenames(
        &entries, 
        &vec_of_strings![
            "somefile",
            "subsubsomefile",
            "subsomefile",
            "smallerfile",
            "subsmallerfile",
            "evensmallerfile",
            "link_somefile"
        ]
    );
}

#[test]
fn test_basic_file_file_order() {
    // Should count:
    // testdir/
    // ├── subdir/
    // │   ├── subsubdir/
    // │   │   ├── subsubsomefile
    // │   ├── link_somefile
    // │   ├── subsmallerfile
    // │   ├── subsomefile
    // ├── evensmallerfile
    // ├── smallerfile
    // └── somefile 1
    let path = Path::new("tests/testdir/somefile");
    let mut lrg = Lrg::new(path, &LrgOptions::default());
    let entries = lrg.sort_descending().get_entries();
    test_entries_against_filenames(
        &entries, 
        &vec_of_strings![
            "somefile"
        ]
    );
}

#[test]
fn test_basic_max_depth_file_order() {
    // Should count:
    // testdir/
    // ├── subdir/
    // │   ├── subsubdir/
    // │   │   ├── subsubsomefile
    // │   ├── link_somefile
    // │   ├── subsmallerfile
    // │   ├── subsomefile
    // ├── evensmallerfile 3
    // ├── smallerfile 2
    // └── somefile 1
    let path = Path::new("tests/testdir");
    let opts = LrgOptions {
        max_depth: 1,
        ..LrgOptions::default()
    };
    let mut lrg = Lrg::new(path, &opts);
    let entries = lrg.sort_descending().get_entries();
    test_entries_against_filenames(
        &entries, 
        &vec_of_strings![
            "somefile",
            "smallerfile",
            "evensmallerfile"
        ]
    );
}

#[test]
fn test_basic_file_and_dir_order() {
    // Should count:
    // testdir/ 7
    // ├── subdir/ 8
    // │   ├── subsubdir/ 9
    // │   │   ├── subsubsomefile 2
    // │   ├── link_somefile 10
    // │   ├── subsmallerfile 5
    // │   ├── subsomefile 3
    // ├── evensmallerfile 6
    // ├── smallerfile 4
    // └── somefile 1
    let path = Path::new("tests/testdir");
    let opts = LrgOptions {
        include_dirs: true,
        ..LrgOptions::default()
    };
    let mut lrg = Lrg::new(path, &opts);
    let entries = lrg.sort_descending().get_entries();
    test_entries_against_filenames(
        &entries, 
        &vec_of_strings![
            "somefile",
            "subsubsomefile",
            "subsomefile",
            "smallerfile",
            "subsmallerfile",
            "evensmallerfile",
            "testdir",
            "subdir",
            "subsubdir",
            "link_somefile"
        ]
    );
}

#[test]
fn test_basic_link_order() {
    // Should count:
    // testdir/
    // ├── subdir/
    // │   ├── subsubdir/
    // │   │   ├── subsubsomefile 3
    // │   ├── link_somefile 2
    // │   ├── subsmallerfile 6
    // │   ├── subsomefile 4
    // ├── evensmallerfile 7
    // ├── smallerfile 5
    // └── somefile 1
    let path = Path::new("tests/testdir");
    let opts = LrgOptions {
        follow_links: true,
        ..LrgOptions::default()
    };
    let mut lrg = Lrg::new(path, &opts);
    let entries = lrg.sort_descending().get_entries();
    test_entries_against_filenames(
        &entries, 
        &vec_of_strings![
            "somefile",
            "link_somefile",
            "subsubsomefile",
            "subsomefile",
            "smallerfile",
            "subsmallerfile",
            "evensmallerfile"
        ]
    );
}

#[test]
fn test_min_depth_order() {
    // Should count:
    // testdir/
    // ├── subdir/
    // │   ├── subsubdir/
    // │   │   ├── subsubsomefile 1
    // │   ├── link_somefile 4
    // │   ├── subsmallerfil 3
    // │   ├── subsomefile 2
    // ├── evensmallerfile
    // ├── smallerfile
    // └── somefile
    let path = Path::new("tests/testdir");
    let opts = LrgOptions {
        min_depth: 2,
        ..LrgOptions::default()
    };
    let mut lrg = Lrg::new(path, &opts);
    let entries = lrg.sort_descending().get_entries();
    test_entries_against_filenames(
        &entries, 
        &vec_of_strings![
            "subsubsomefile",
            "subsomefile",
            "subsmallerfile",
            "link_somefile"
        ]
    );
}