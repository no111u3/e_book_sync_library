use std::fs;
use std::path::{Path, PathBuf};

use crate::bookshelf::Bookshelf;
use crate::indexer::Indexer;

pub struct Updater {
    local: String,
    foreign: String,
}

pub enum Update {
    OnlyFromLocal,
    OnlyFromForeign,
    Bidirectional,
}

impl Updater {
    pub fn new(local: String, foreign: String) -> Self {
        Updater { local, foreign }
    }

    fn scan_area(&self) -> (Bookshelf, Bookshelf) {
        (
            Indexer::new(self.local.clone()).index(),
            Indexer::new(self.foreign.clone()).index(),
        )
    }

    fn cross_diff(&self, (local, foreign): (Bookshelf, Bookshelf)) -> (Bookshelf, Bookshelf) {
        (local.difference(&foreign), foreign.difference(&local))
    }

    fn copy_files(&self, books: Bookshelf, destination: PathBuf) {
        for b in books.iter() {
            let mut dest_path = destination.to_path_buf();
            dest_path.push(b.get_path().strip_prefix(books.get_path()).unwrap());
            let dest_path_dir = dest_path.parent().unwrap();

            if !dest_path_dir.exists() {
                fs::create_dir(dest_path_dir);
            }

            match fs::copy(b.get_path(), dest_path) {
                Err(e) => panic!("Copy error: {}", e),
                Ok(_) => (),
            }
        }
    }

    pub fn update(&self, update: Update) {
        let (from_local, from_foreign) = self.cross_diff(self.scan_area());
        match update {
            Update::OnlyFromLocal => {
                self.copy_files(from_local, PathBuf::from(self.foreign.clone()))
            }
            Update::OnlyFromForeign => {
                self.copy_files(from_foreign, PathBuf::from(self.local.clone()))
            }
            Update::Bidirectional => {
                self.copy_files(from_local, PathBuf::from(self.foreign.clone()));
                self.copy_files(from_foreign, PathBuf::from(self.local.clone()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::Updater;
    use crate::book::Book;

    #[test]
    fn scan_area() {
        let uper = Updater::new(
            "tests/scan_area/local".to_string(),
            "tests/scan_area/foreign".to_string(),
        );

        let (local, foreign) = uper.scan_area();

        let ixer_res: Vec<_> = local.iter().cloned().collect();
        assert_eq!(
            ixer_res,
            [
                Book::new(String::from("file_one.txt")),
                Book::new(String::from("file_three.txt")),
                Book::new(String::from("file_two.txt")),
            ]
        );

        let ixer_res: Vec<_> = foreign.iter().cloned().collect();
        assert_eq!(
            ixer_res,
            [
                Book::new(String::from("file_four.txt")),
                Book::new(String::from("file_one.txt")),
                Book::new(String::from("file_two.txt")),
            ]
        );
    }

    #[test]
    fn cross_diff() {
        let uper = Updater::new(
            "tests/scan_area/local".to_string(),
            "tests/scan_area/foreign".to_string(),
        );

        let (from_local, from_foreign) = uper.cross_diff(uper.scan_area());

        let ixer_res: Vec<_> = from_local.iter().cloned().collect();
        assert_eq!(ixer_res, [Book::new(String::from("file_three.txt")),]);

        let ixer_res: Vec<_> = from_foreign.iter().cloned().collect();
        assert_eq!(ixer_res, [Book::new(String::from("file_four.txt")),]);
    }

    #[test]
    fn copy_files() {
        let uper = Updater::new(
            "tests/copy_files/local".to_string(),
            "tests/copy_files/foreign".to_string(),
        );

        let delete: std::io::Result<()> = (|| {
            fs::remove_file("tests/copy_files/foreign/file_one.txt")?;
            fs::remove_file("tests/copy_files/foreign/file_two.txt")?;
            fs::remove_file("tests/copy_files/foreign/file_three.txt")?;
            fs::remove_file("tests/copy_files/foreign/test/file_four.txt")?;
            fs::remove_dir("tests/copy_files/foreign/test")?;
            Ok(())
        })();
        match delete {
            Err(e) => panic!("Delete error: {}", e),
            Ok(_) => (),
        }

        let (from_local, from_foreign) = uper.cross_diff(uper.scan_area());

        let ixer_res: Vec<_> = from_local.iter().cloned().collect();
        assert_eq!(
            ixer_res,
            [
                Book::new(String::from("file_four.txt")),
                Book::new(String::from("file_one.txt")),
                Book::new(String::from("file_three.txt")),
                Book::new(String::from("file_two.txt")),
            ]
        );

        uper.copy_files(from_local, from_foreign.get_path().to_path_buf());

        let (from_local, from_foreign) = uper.cross_diff(uper.scan_area());

        let ixer_res: Vec<_> = from_local.iter().cloned().collect();
        assert_eq!(ixer_res, []);
    }
}
