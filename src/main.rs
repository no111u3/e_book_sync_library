mod book;
mod bookshelf;
mod indexer;
mod updater;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("Sync your e-book library")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();
}
