
use std::env;
use std::process;
use std::path::{PathBuf};

use lrg::{Lrg, Options};

use humansize::{FileSize, file_size_opts as options};

// TODO non-recursive
// TODO colored output

fn main() {
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

    // use humansize::{FileSize, file_size_opts as options};
    // println!("{}: {}", size.file_size(options::CONVENTIONAL).unwrap(), dir_entry.path().display());

    for entry in entries[0..5].to_vec() {
        println!("{}: {}", entry.metadata().expect("meta").len().file_size(options::CONVENTIONAL).unwrap(), entry.path().display());
    }
}
