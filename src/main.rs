
extern crate walkdir;
extern crate humansize;

use std::io::{ErrorKind};
use std::env;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use humansize::{FileSize, file_size_opts as options};

fn main() {

    let args: Vec<String> = env::args().collect();
    
    // TODO put all types of error in same handler

    let current_dir = if args.len() == 0 || args.len() == 1 {
        env::current_dir().expect("current dir")
    } else {
        PathBuf::from(&args[1])
    };

    let mut map = BTreeMap::new();

    for entry in WalkDir::new(&current_dir) {
        match entry {
            Ok(entry) => {
                match entry.metadata() {
                    Ok(metadata) => map.insert(metadata.len(), entry.clone()),
                    Err(err) => {
                        // Because ErrorKind.as_str() is private
                        let error_message = match err.kind() {
                            ErrorKind::NotFound => "Entity not found",
                            ErrorKind::PermissionDenied => "Permission denied",
                            ErrorKind::ConnectionRefused => "Connection refused",
                            ErrorKind::ConnectionReset => "Connection reset",
                            ErrorKind::ConnectionAborted => "Connection aborted",
                            ErrorKind::NotConnected => "Not connected",
                            ErrorKind::AddrInUse => "Address in use",
                            ErrorKind::AddrNotAvailable => "Address not available",
                            ErrorKind::BrokenPipe => "Broken pipe",
                            ErrorKind::AlreadyExists => "Entity already exists",
                            ErrorKind::WouldBlock => "Operation would block",
                            ErrorKind::InvalidInput => "Invalid input parameter",
                            ErrorKind::InvalidData => "Invalid data",
                            ErrorKind::TimedOut => "Timed out",
                            ErrorKind::WriteZero => "Write zero",
                            ErrorKind::Interrupted => "Operation interrupted",
                            ErrorKind::Other => "Other os error",
                            ErrorKind::UnexpectedEof => "Unexpected end of file",
                            _ => "Unknown error",
                        };
                        println!("lrg: couldn't parse metadata of {}: {}", entry.path(), error_message);
                    }

                }
            },
            Err(err) => {
                let path = err.path().unwrap_or(Path::new("")).display();
                let error = match err.io_error() {
                    Some(err) => {
                        // Because ErrorKind.as_str() is private
                        match err.kind() {
                            ErrorKind::NotFound => "Entity not found",
                            ErrorKind::PermissionDenied => "Permission denied",
                            ErrorKind::ConnectionRefused => "Connection refused",
                            ErrorKind::ConnectionReset => "Connection reset",
                            ErrorKind::ConnectionAborted => "Connection aborted",
                            ErrorKind::NotConnected => "Not connected",
                            ErrorKind::AddrInUse => "Address in use",
                            ErrorKind::AddrNotAvailable => "Address not available",
                            ErrorKind::BrokenPipe => "Broken pipe",
                            ErrorKind::AlreadyExists => "Entity already exists",
                            ErrorKind::WouldBlock => "Operation would block",
                            ErrorKind::InvalidInput => "Invalid input parameter",
                            ErrorKind::InvalidData => "Invalid data",
                            ErrorKind::TimedOut => "Timed out",
                            ErrorKind::WriteZero => "Write zero",
                            ErrorKind::Interrupted => "Operation interrupted",
                            ErrorKind::Other => "Other os error",
                            ErrorKind::UnexpectedEof => "Unexpected end of file",
                            _ => "Unknown error",
                        }
                    },
                    None => "Unknown error",
                };
                println!("lrg: cannot access `{}`: {}", path, error);
            }
        }
    }


    for (i, (size, dir_entry)) in map.iter().rev().enumerate() {
        if i == 5 {
            break;
        }
        println!("{}: {}", size.file_size(options::CONVENTIONAL).unwrap(), dir_entry.path().display());
    }
}
