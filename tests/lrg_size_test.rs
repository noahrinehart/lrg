extern crate lrg;

use std::path::Path;
use lrg::{Lrg, LrgOptions, DirEntry};


fn test_entries_against_sizes(entries: &Vec<DirEntry>, sizes: &Vec<u64>) {
    assert_eq!(entries.len(), sizes.len());
    for (i, entry) in entries.iter().enumerate() {
        // println!("{:?}", entry.metadata().expect("Cannot get filesize").len());
        assert_eq!(entry.metadata().expect("Cannot get filesize").len(), sizes[i]);
    }
}


#[test]
fn test_basic_dir_file_sizes() {
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
    test_entries_against_sizes(
        &entries,
        &vec![
            1024000,
            204800,
            102400,
            51200,
            20480,
            10240,
            11,
        ]
    );
}

#[test]
fn test_basic_file_file_size() {
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
    test_entries_against_sizes(
        &entries,
        &vec![
            1024000,
        ]
    );
}

#[test]
fn test_basic_max_depth_file_size() {
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
    test_entries_against_sizes(
        &entries, 
        &vec![
            1024000,
            51200,
            10240,
        ]
    );
}

#[test]
fn test_basic_file_and_dir_size() {
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
    test_entries_against_sizes(
        &entries,
        &vec![
            1024000,
            204800,
            102400,
            51200,
            20480,
            10240,
            4096,
            4096,
            4096,
            11,
        ]
    );
}

#[test]
fn test_basic_link_size() {
    // Should count:
    // testdir/
    // ├── subdir/
    // │   ├── subsubdir/
    // │   │   ├── subsubsomefile 2
    // │   ├── link_somefile 1
    // │   ├── subsmallerfile 5
    // │   ├── subsomefile 3
    // ├── evensmallerfile 6
    // ├── smallerfile 4
    // └── somefile 1
    let path = Path::new("tests/testdir");
    let opts = LrgOptions {
        follow_links: true,
        ..LrgOptions::default()
    };
    let mut lrg = Lrg::new(path, &opts);
    let entries = lrg.sort_descending().get_entries();
    test_entries_against_sizes(
        &entries,
        &vec![
            1024000,
            1024000,
            204800,
            102400,
            51200,
            20480,
            10240,
        ]
    );
}

#[test]
fn test_min_depth_size() {
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
    test_entries_against_sizes(
        &entries,
        &vec![
            204800,
            102400,
            20480,
            11,
        ]
    )
}