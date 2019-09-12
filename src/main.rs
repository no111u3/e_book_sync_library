mod book;
mod bookshelf;
mod indexer;
mod updater;

use std::path::PathBuf;

use clap::{App, Arg};

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
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("destination")
                .short("d")
                .long("dst")
                .value_name("DST")
                .help("Destination directory, e-ink device folder")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let source = matches.value_of("source").unwrap();
    let destination = matches.value_of("destination").unwrap();

    println!("Sync: local::{} <-> device::{}", source, destination);

    let uper = Updater::new(source.to_string(), destination.to_string());

    let path_src = PathBuf::from(source);
    let path_dst = PathBuf::from(destination);

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
                .strip_prefix(path_src.clone())
                .unwrap()
                .to_str()
                .unwrap(),
            book_status
                .get_dst()
                .strip_prefix(path_dst.clone())
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
                BookCopyMoveStatus::Movied => String::from("Movied"),
                BookCopyMoveStatus::NotMoviedWithError(e) => format!("Move error: {}", e),
                _ => String::from("Unexpected error"),
            },
            book_status
                .get_src()
                .strip_prefix(path_src.clone())
                .unwrap()
                .to_str()
                .unwrap(),
            book_status
                .get_dst()
                .strip_prefix(path_src.clone())
                .unwrap()
                .to_str()
                .unwrap()
        );
    }
}
