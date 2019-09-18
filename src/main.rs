mod book;
mod bookshelf;
mod config;
mod indexer;
mod updater;

use std::env;
use std::path::PathBuf;
use std::process;

use clap::{App, Arg};

use config::Config;
use updater::{BookCopyMoveStatus, Update, Updater};

fn main() {
    let matches = App::new("Sync your e-book library")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("source")
                .short("s")
                .long("src")
                .value_name("SRC")
                .help("Source sync directory, local folder")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("destination")
                .short("d")
                .long("dst")
                .value_name("DST")
                .help("Destination directory, e-ink device folder")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("cfg")
                .value_name("CONFIG.yaml")
                .help("Program configuration file with source/destination sync folders")
                .takes_value(true),
        )
        .get_matches();

    let (source, destination) = match (matches.value_of("source"), matches.value_of("destination"))
    {
        (Some(source), Some(destination)) => (PathBuf::from(source), PathBuf::from(destination)),
        (_, _) => {
            println!("Parse config to extract source/destination paths");

            let path = match matches.value_of("config") {
                Some(config) => PathBuf::from(config),
                _ => {
                    let mut default_path = PathBuf::from(env::var("HOME").unwrap());
                    default_path.push(".config");
                    default_path.push(env!("CARGO_PKG_NAME"));
                    default_path.push("config.yaml");

                    default_path
                }
            };

            if !path.exists() {
                println!("Config: {} doesn't exist", path.to_str().unwrap());
                process::exit(1);
            }

            let config = Config::new(path);

            config.parse()
        }
    };

    println!(
        "Sync: local::{} <-> device::{}",
        source.to_str().unwrap(),
        destination.to_str().unwrap()
    );

    if !source.exists() {
        println!("Source path: {} doesn't exist", source.to_str().unwrap());
        process::exit(1);
    }

    if !destination.exists() {
        println!(
            "Source path: {} doesn't exist",
            destination.to_str().unwrap()
        );
        process::exit(1);
    }

    let uper = Updater::new(source.clone(), destination.clone());

    for book_status in uper.update(Update::OnlyFromLocal) {
        println!(
            "{} {} => from: {} to: {}",
            book_status.get_name(),
            match book_status.get_status() {
                BookCopyMoveStatus::Copied => String::from("Copied"),
                BookCopyMoveStatus::NotCopiedWithError(e) => format!("Copy error: {}", e),
                _ => String::from("Unexpected error"),
            },
            book_status
                .get_src()
                .strip_prefix(&source)
                .unwrap()
                .to_str()
                .unwrap(),
            book_status
                .get_dst()
                .strip_prefix(&destination)
                .unwrap()
                .to_str()
                .unwrap()
        );
    }

    for book_status in uper.update(Update::OnlyFromForeignSync) {
        println!(
            "{} {} <= from: {} to: {}",
            book_status.get_name(),
            match book_status.get_status() {
                BookCopyMoveStatus::Moved => String::from("Movied"),
                BookCopyMoveStatus::NotMovedWithError(e) => format!("Move error: {}", e),
                _ => String::from("Unexpected error"),
            },
            book_status
                .get_src()
                .strip_prefix(&source)
                .unwrap()
                .to_str()
                .unwrap(),
            book_status
                .get_dst()
                .strip_prefix(&source)
                .unwrap()
                .to_str()
                .unwrap()
        );
    }
}
