
use std::env;
use std::process;
use std::path::{PathBuf};

use lrg::{Lrg, Options, get_walkdir_error_str};

use humansize::{FileSize, file_size_opts as options};

// TODO non-recursive
// TODO customize format
// TODO colored output

fn main() {
    // Init env_logger
    env_logger::init();

    // Get args
    let args: Vec<String> = env::args().collect();

    // Get current directory (1st param or current directory)
    let current_dir = if args.len() == 0 || args.len() == 1 {
        // If program path is only argument
        match env::current_dir() {
            Ok(path) => path.as_path().to_owned(),
            _ => {
                println!("Error: couldn't get current directory");
                process::exit(1);
            }
        }
    } else {
        // If argument supplied
        PathBuf::from(&args[1]).as_path().to_owned()
    };

    // Fetch entries 
    let entries = Lrg::new(&current_dir, &Options::default()).sort_descending().get_entries();

    // Options for printing humansize'd numbers
    let hs_options = options::FileSizeOpts {
        allow_negative: true,
        ..options::CONVENTIONAL
    };

    // Iterate through entries
    for (i, entry) in entries.iter().enumerate() {
        // Break at number of requested entries
        if i == 5 {
            break;
        }
        
        // Handle error getting meetadata
        match entry.metadata() {
            Ok(meta) => {
                // Unwrap since guranteed to not panic due to options
                println!("{}: {}", meta.len().file_size(&hs_options).unwrap(), entry.path().display());
            },
            Err(err) => {
                let error_message = get_walkdir_error_str(&err);
                println!("lrg: cannot get metadata of '{}': {}", entry.path().display(), error_message);
            },
        }
    }
}
