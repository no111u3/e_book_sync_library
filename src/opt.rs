//! Opt entity
//!
//! Command line options as StructOpt derive structure
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = env!("CARGO_PKG_NAME"), about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct Opt {
    /// Source sync directory, local folder
    #[structopt(short, long, parse(from_os_str))]
    pub source: Option<PathBuf>,

    /// Destination directory, e-ink device folder
    #[structopt(short, long, parse(from_os_str))]
    pub destination: Option<PathBuf>,

    /// Program configuration file with source/destination sync folders
    #[structopt(short, long, parse(from_os_str))]
    pub config: Option<PathBuf>,
}
