extern crate lrg;

use std::path::Path;
use lrg::{Lrg, LrgOptions};

#[test]
fn test_basic_dir_file_count() {
    // Should count:
    // testdir/
    // ├── subdir/
    // │   ├── subsubdir/
    // │   │   ├── subsubsomefile *
    // │   ├── link_somefile *
    // │   ├── subsmallerfile *
    // │   ├── subsomefile *
    // ├── evensmallerfile *
    // ├── smallerfile *
    // └── somefile *
    let path = Path::new("tests/testdir");
    let lrg = Lrg::new(path, &LrgOptions::default());
    assert_eq!(7, lrg.get_entries().len());
}

#[test]
fn test_basic_file_file_count() {
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
    // └── somefile *
    let path = Path::new("tests/testdir/somefile");
    let lrg = Lrg::new(path, &LrgOptions::default());
    assert_eq!(1, lrg.get_entries().len());
}

#[test]
fn test_basic_max_depth_count() {
    // Should count:
    // testdir/
    // ├── subdir/
    // │   ├── subsubdir/
    // │   │   ├── subsubsomefile
    // │   ├── link_somefile
    // │   ├── subsmallerfile
    // │   ├── subsomefile
    // ├── evensmallerfile *
    // ├── smallerfile *
    // └── somefile *
    let path = Path::new("tests/testdir");
    let opts = LrgOptions {
        max_depth: 1,
        ..LrgOptions::default()
    };
    let lrg = Lrg::new(path, &opts);
    assert_eq!(3, lrg.get_entries().len());
}

#[test]
fn test_basic_file_and_dir_count() {
    // Should count:
    // testdir/ *
    // ├── subdir/ *
    // │   ├── subsubdir/ *
    // │   │   ├── subsubsomefile *
    // │   ├── link_somefile *
    // │   ├── subsmallerfile *
    // │   ├── subsomefile *
    // ├── evensmallerfile *
    // ├── smallerfile *
    // └── somefile *
    let path = Path::new("tests/testdir");
    let opts = LrgOptions {
        include_dirs: true,
        ..LrgOptions::default()
    };
    let lrg = Lrg::new(path, &opts);
    assert_eq!(10, lrg.get_entries().len());
}

#[test]
fn test_basic_link_count() {
    // Should count:
    // testdir/
    // ├── subdir/
    // │   ├── subsubdir/
    // │   │   ├── subsubsomefile *
    // │   ├── link_somefile *
    // │   ├── subsmallerfile *
    // │   ├── subsomefile *
    // ├── evensmallerfile *
    // ├── smallerfile *
    // └── somefile *
    let path = Path::new("tests/testdir");
    let opts = LrgOptions {
        follow_links: true,
        ..LrgOptions::default()
    };
    let lrg = Lrg::new(path, &opts);
    assert_eq!(7, lrg.get_entries().len());
}

#[test]
fn test_min_depth_count() {
    // Should count:
    // testdir/
    // ├── subdir/
    // │   ├── subsubdir/
    // │   │   ├── subsubsomefile *
    // │   ├── link_somefile *
    // │   ├── subsmallerfile *
    // │   ├── subsomefile *
    // ├── evensmallerfile
    // ├── smallerfile
    // └── somefile
    let path = Path::new("tests/testdir");
    let opts = LrgOptions {
        min_depth: 2,
        ..LrgOptions::default()
    };
    let lrg = Lrg::new(path, &opts);
    assert_eq!(4, lrg.get_entries().len());
}