use std::env;
use std::path::PathBuf;
use std::process;

use dirs::config_dir;

use structopt::StructOpt;

use e_book_sync_library::config::Config;
use e_book_sync_library::opt::Opt;
use e_book_sync_library::updater::{Update, Updater};

fn main() {
    let opt = Opt::from_args();

    let (source, destination) = match (opt.source, opt.destination) {
        (Some(source), Some(destination)) => (source, destination),
        (_, _) => {
            println!("Parse config to extract source/destination paths");

            let path = match opt.config {
                Some(config) => PathBuf::from(config),
                _ => {
                    let mut default_path = PathBuf::from(config_dir().unwrap());
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

            match config.parse() {
                Ok(paths) => paths,
                Err(e) => {
                    println!("Error for parse config: {}", e);
                    process::exit(1);
                }
            }
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
            "Destination path: {} doesn't exist",
            destination.to_str().unwrap()
        );
        process::exit(1);
    }

    let updater = Updater::new(source.clone(), destination.clone());

    for book_status in updater.update(Update::OnlyFromForeignSync) {
        println!(
            "{} {} <= from: {} to: {}",
            book_status.get_name(),
            book_status.get_status(),
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

    for book_status in updater.update(Update::OnlyFromLocal) {
        println!(
            "{} {} +> from: {} to: {}",
            book_status.get_name(),
            book_status.get_status(),
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

    for book_status in updater.update(Update::OnlyFromForeign) {
        println!(
            "{} {} <+ from: {} to: {}",
            book_status.get_name(),
            book_status.get_status(),
            book_status
                .get_src()
                .strip_prefix(&destination)
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
