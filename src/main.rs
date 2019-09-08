mod book;
mod bookshelf;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("Sync your e-book library")
        .version("0.1.0")
        .author("Boris V. <no111u3@gmail.com>")
        .about("Synchonize e-book with your local e-library")
        .get_matches();
}
