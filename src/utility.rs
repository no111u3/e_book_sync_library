//! Utility units
//!
//! Service and utitility units

use std::fs;
use std::io;
use std::path::PathBuf;

pub fn create_dir_for_path(path: &PathBuf) -> io::Result<()> {
    let path_dir = path.parent().unwrap();

    if !path_dir.exists() {
        match create_dir_for_path(&path_dir.to_path_buf()) {
            Ok(_) => fs::create_dir(path_dir),
            Err(x) => Err(x),
        }
    } else {
        Ok(())
    }
}
