extern crate lrg;
extern crate clap;
extern crate humansize;
extern crate pathdiff;

use std::env;
use std::path::{PathBuf};
use std::process;

use lrg::{get_walkdir_error_str, Lrg, LrgOptions, SortBy};

use clap::{App, Arg};
use humansize::{file_size_opts, FileSize};
use pathdiff::diff_paths;

// TODO colored output
// TODO handle errors function options for lib

fn main() {
    // Init env_logger
    env_logger::init();

    // Get args
    let matches = App::new("lrg")
        .version("0.2.0")
        .author("Noah Rinehart <rinehart.noah@gmail.com>")
        .about("A utility to help find the largest file(s) in a directory")
        .arg(Arg::with_name("NUMBER")
            .short("n")
            .long("number")
            .value_name("NUM_ENTRIES")
            .help("sets the number of files to list (default: 5)")
            .takes_value(true))
        .arg(Arg::with_name("RECURSIVE")
            .short("r")
            .long("no-recursion")
            .help("will only visit files in specified directory, takes precedence over max-depth (default: false)"))
        .arg(Arg::with_name("MAX_DEPTH")
            .short("d")
            .long("max-depth")
            .value_name("MAX_DEPTH")
            .help("sets the maximum depth of folders to search, unless --no-recursion specified (default: max possible)")
            .takes_value(true))
        .arg(Arg::with_name("FOLLOW_LINKS")
            .short("l")
            .long("follow-links")
            .help("will follow links of files (default: false)"))
        .arg(Arg::with_name("DIRECTORIES")
            .short("i")
            .long("directories")
            .help("include directories in search (default: false)"))
        .arg(Arg::with_name("ASCENDING")
            .short("a")
            .long("ascending")
            .help("sort the results in ascending order (default: false)"))
        .arg(Arg::with_name("ABSOLUTE")
            .short("b")
            .long("absolute")
            .help("outputs files' absolute path (default: false)"))
        .arg(Arg::with_name("UNITS")
            .short("u")
            .long("units")
            .value_name("UNITS")
            .help("sets the units to display: decimal for 1000KB, binary for 1024KiB, conventional for 1024KB (default: conventional)")
            .takes_value(true))
        .arg(Arg::with_name("FILEPATH")
            .help("the path to search in")
            .index(1))
        .get_matches();

    // Get directory to search
    let current_dir = match matches.value_of("FILEPATH") {
        Some(filepath) => PathBuf::from(filepath),
        None => match env::current_dir() {
            Ok(path) => path.as_path().to_owned(),
            Err(_err) => {
                println!("Error: couldn't get current directory");
                process::exit(1);
            }
        },
    };

    // Get number of files to get
    let num_entries = match matches.value_of("NUMBER") {
        Some(number) => match number.parse::<usize>() {
            Ok(number) => number,
            Err(_err) => {
                println!("Error: couldn't parse number of files to list");
                process::exit(1);
            }
        },
        None => 5,
    };

    // Get max depth of find
    let max_depth = if matches.is_present("RECURSIVE") {
        1
    } else {
        match matches.value_of("MAX_DEPTH") {
            Some(depth) => match depth.parse::<usize>() {
                Ok(depth) => depth,
                Err(_err) => {
                    println!("Error: couldn't parse max depth");
                    process::exit(1);
                }
            },
            None => ::std::usize::MAX,
        }
    };

    // Whether to follow links or not
    let follow_links = matches.is_present("FOLLOW_LINKS");

    // Whether to include directories or not
    let include_dirs = matches.is_present("DIRECTORIES");

    // Whether to sort by ascending or not
    let sort_value = if matches.is_present("ASCENDING") {
        SortBy::Ascending
    } else {
        SortBy::Descending
    };

    // Whether to output absolute or relative values
    let output_absolute = matches.is_present("ABSOLUTE");

    // Parse units to use when printing
    let units = match matches.value_of("UNITS") {
        Some(unit) => {
            match unit {
                "decimal" => file_size_opts::DECIMAL,
                "binary" => file_size_opts::BINARY,
                "conventional" => file_size_opts::CONVENTIONAL,
                _ => {
                    println!("Error: couldn't parse units");
                    process::exit(1);
                },
            }
        },
        None => file_size_opts::CONVENTIONAL,
    };

    // Set options for finding entries
    let options = LrgOptions {
        max_depth,
        follow_links,
        include_dirs,
        ..LrgOptions::default()
    };

    // Fetch entries
    let entries = Lrg::new(&current_dir, &options)
        .sort_by(&sort_value)
        .get_entries();

    // Check for no entries found
    if entries.is_empty() {
        println!("lrg: no files found");
        process::exit(1);
    }

    // Options for printing humansize'd numbers
    let hs_options = file_size_opts::FileSizeOpts {
        allow_negative: true,
        ..units
    };

    // Iterate through entries
    for (i, entry) in entries.iter().enumerate() {
        // Break at number of requested entries
        if i == num_entries {
            break;
        }

        // Get the path to display depending on flags
        let display_path = if output_absolute {
            format!("{}", entry.path().display())
        } else {
            format!("{}", diff_paths(entry.path(), &current_dir).unwrap_or_else(|| PathBuf::new()).display())

        };

        // Handle error getting metadata
        match entry.metadata() {
            Ok(meta) => {
                // Unwrap since guranteed to not panic due to options
                println!(
                    "{}: {}",
                    meta.len().file_size(&hs_options).unwrap(),
                    display_path
                );
            }
            Err(err) => {
                let error_message = get_walkdir_error_str(&err);
                println!(
                    "lrg: cannot get metadata of '{}': {}",
                    display_path,
                    error_message
                );
            }
        }
    }
}
