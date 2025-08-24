//! Small port of the ls command using clap builder

use crate::metadata;
use clap::{ArgMatches, arg, command};
use std::{
    fs::{self, ReadDir},
    process,
};

fn get_args() -> ArgMatches {
    command!()
        .arg(arg!([PATH]).default_value("."))
        .arg(arg!(longlisting: -l "use a long listing format"))
        .get_matches()
}

fn get_dir_entries(matches: &ArgMatches) -> ReadDir {
    let path = matches
        .get_one::<String>("PATH")
        .expect("Clap should always provide a default value");

    let Ok(entries) = fs::read_dir(path) else {
        eprintln!("Could not open path {path}");
        process::exit(1);
    };
    entries
}

fn print_metadata(entry: &fs::DirEntry) {
    let Ok(metadata) = entry.metadata() else {
        eprintln!(
            "Could not get metadata from {}",
            entry.file_name().display()
        );
        process::exit(-2);
    };

    let permissions = match metadata::parse_permissions(metadata.permissions()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{e}");
            process::exit(-3)
        }
    };

    print!("{permissions}\t");
}

pub fn run() {
    let matches = get_args();
    let entries = get_dir_entries(&matches);

    for entry in entries {
        let Ok(entry) = entry else {
            eprintln!("Could not read file");
            process::exit(-1);
        };

        if matches.get_flag("longlisting") {
            print_metadata(&entry);
        }

        println!("{}", entry.file_name().display());
    }
}
