//! Indexer entity
//!
//! Create file index, parsing through walking directory

use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

use crate::book::Book;
use crate::bookshelf::Bookshelf;

pub struct Indexer {
    path: PathBuf,
}

impl Indexer {
    pub fn new(path: PathBuf) -> Self {
        Indexer { path }
    }

    pub fn index(&self) -> Bookshelf {
        WalkDir::new(self.path.clone())
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .fold(Bookshelf::from(self.path.clone()), |mut bs, entry| {
                bs.add(Book::from(entry.path().to_path_buf()));
                bs
            })
    }

    pub fn index_map(&self, map: fn(de: &DirEntry)) -> Bookshelf {
        WalkDir::new(self.path.clone())
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| {
                map(&e);
                e
            })
            .fold(Bookshelf::from(self.path.clone()), |mut bs, entry| {
                bs.add(Book::from(entry.path().to_path_buf()));
                bs
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterate() {
        let ixer = Indexer::new(PathBuf::from("tests/iterate"));
        let ixer_res: Vec<_> = ixer.index().iter().cloned().collect();
        assert_eq!(
            ixer_res,
            [
                Book::new(String::from("file_one.txt")),
                Book::new(String::from("file_three.txt")),
                Book::new(String::from("file_two.txt")),
            ]
        );
    }

    #[test]
    fn iterate_with_map() {
        let ixer = Indexer::new(PathBuf::from("tests/iterate"));
        let ixer_res: Vec<_> = ixer
            .index_map(|de| assert_eq!(de.path().exists(), true))
            .iter()
            .cloned()
            .collect();
        assert_eq!(
            ixer_res,
            [
                Book::new(String::from("file_one.txt")),
                Book::new(String::from("file_three.txt")),
                Book::new(String::from("file_two.txt")),
            ]
        );
    }
}
