use std::fs;
use std::path::{PathBuf};

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

#[derive(Debug, PartialEq, Clone)]
pub enum BookCopyMoveStatus {
    Copied,
    NotCopiedWithError(String),
    Movied,
    NotMoviedWithError(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct BookStatus {
    name: String,
    src: PathBuf,
    dst: PathBuf,
    status: BookCopyMoveStatus,
}

impl BookStatus {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_src(&self) -> &PathBuf {
        &self.src
    }

    pub fn get_dst(&self) -> &PathBuf {
        &self.dst
    }

    pub fn get_status(&self) -> &BookCopyMoveStatus {
        &self.status
    }
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

    fn copy_files(&self, books: Bookshelf, destination: PathBuf) -> Vec<BookStatus> {
        books
            .iter()
            .map(|b| {
                let mut dest_path = destination.to_path_buf();
                dest_path.push(b.get_path().strip_prefix(books.get_path()).unwrap());
                let dest_path_dir = dest_path.parent().unwrap();

                if !dest_path_dir.exists() {
                    fs::create_dir(dest_path_dir);
                }

                BookStatus {
                    name: b.get_name().to_string(),
                    src: b.get_path().to_path_buf(),
                    dst: dest_path.to_path_buf(),
                    status: match fs::copy(b.get_path(), dest_path) {
                        Err(e) => BookCopyMoveStatus::NotCopiedWithError(e.to_string()),
                        Ok(_) => BookCopyMoveStatus::Copied,
                    },
                }
            })
            .collect()
    }

    fn move_files(&self, books_src: Bookshelf, books_dst: Bookshelf) -> Vec<BookStatus> {
        books_src
            .iter()
            .zip(books_dst.iter())
            .filter(|books_to_alloved| {
                let (book_src, book_dst) = books_to_alloved;
                if book_src.get_path().strip_prefix(books_src.get_path())
                    != book_dst.get_path().strip_prefix(books_dst.get_path())
                {
                    true
                } else {
                    false
                }
            })
            .map(|books_to_move| {
                let (book_src, book_dst) = books_to_move;

                let mut dest_path = books_dst.get_path().to_path_buf();
                dest_path.push(
                    book_src
                        .get_path()
                        .strip_prefix(books_src.get_path())
                        .unwrap(),
                );
                let dest_path_dir = dest_path.parent().unwrap();

                if !dest_path_dir.exists() {
                    fs::create_dir(dest_path_dir);
                }

                BookStatus {
                    name: book_src.get_name().to_string(),
                    src: book_dst.get_path().to_path_buf(),
                    dst: dest_path.to_path_buf(),
                    status: {
                        
                        match fs::rename(book_dst.get_path(), dest_path) {
                            Err(e) => BookCopyMoveStatus::NotMoviedWithError(e.to_string()),
                            Ok(_) => BookCopyMoveStatus::Movied,
                        }
                    },
                }
            })
            .collect()
    }

    pub fn update(&self, update: Update) -> Vec<BookStatus> {
        let (from_local, from_foreign) = self.cross_diff(self.scan_area());
        match update {
            Update::OnlyFromLocal => {
                self.copy_files(from_local, PathBuf::from(self.foreign.clone()))
            }
            Update::OnlyFromForeign => {
                self.copy_files(from_foreign, PathBuf::from(self.local.clone()))
            }
            Update::Bidirectional => {
                let mut results_of_copy: Vec<BookStatus> = Vec::new();
                results_of_copy
                    .append(&mut self.copy_files(from_local, PathBuf::from(self.foreign.clone())));
                results_of_copy
                    .append(&mut self.copy_files(from_foreign, PathBuf::from(self.local.clone())));
                results_of_copy
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::BookCopyMoveStatus;
    use super::Update;
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

        let results_of_copy = uper
            .copy_files(from_local, from_foreign.get_path().to_path_buf())
            .iter()
            .map(|e| (e.get_name().to_string(), e.get_status().clone()))
            .collect::<Vec<(String, BookCopyMoveStatus)>>();
        assert_eq!(
            results_of_copy,
            [
                (String::from("file_four.txt"), BookCopyMoveStatus::Copied),
                (String::from("file_one.txt"), BookCopyMoveStatus::Copied),
                (String::from("file_three.txt"), BookCopyMoveStatus::Copied),
                (String::from("file_two.txt"), BookCopyMoveStatus::Copied),
            ]
        );

        let (from_local, _) = uper.cross_diff(uper.scan_area());

        let ixer_res: Vec<_> = from_local.iter().cloned().collect();
        assert_eq!(ixer_res, []);
    }

    #[test]
    fn move_files() {
        let uper = Updater::new(
            "tests/move_files/local".to_string(),
            "tests/move_files/foreign".to_string(),
        );

        let delete: std::io::Result<()> = (|| {
            fs::rename(
                "tests/move_files/foreign/test/file_four.txt",
                "tests/move_files/foreign/file_four.txt",
            )?;
            fs::remove_dir("tests/move_files/foreign/test")?;
            Ok(())
        })();
        match delete {
            Err(e) => panic!("Delete error: {}", e),
            Ok(_) => (),
        }

        let (from_local, from_foreign) = uper.scan_area();

        let results_of_move = uper
            .move_files(from_local, from_foreign)
            .iter()
            .map(|e| (e.get_name().to_string(), e.get_status().clone()))
            .collect::<Vec<(String, BookCopyMoveStatus)>>();
        assert_eq!(
            results_of_move,
            [(String::from("file_four.txt"), BookCopyMoveStatus::Movied),]
        );
    }

    #[test]
    fn update_files() {
        let uper = Updater::new(
            "tests/update_files/local".to_string(),
            "tests/update_files/foreign".to_string(),
        );

        let delete: std::io::Result<()> = (|| {
            fs::remove_file("tests/update_files/foreign/file_three.txt")?;
            fs::remove_file("tests/update_files/local/file_four.txt")?;
            Ok(())
        })();
        match delete {
            Err(e) => panic!("Delete error: {}", e),
            Ok(_) => (),
        }

        let (from_local, from_foreign) = uper.cross_diff(uper.scan_area());

        let ixer_res: Vec<_> = from_local.iter().cloned().collect();
        assert_eq!(ixer_res, [Book::new(String::from("file_three.txt")),]);

        let ixer_res: Vec<_> = from_foreign.iter().cloned().collect();
        assert_eq!(ixer_res, [Book::new(String::from("file_four.txt")),]);

        let results_of_copy = uper
            .update(Update::Bidirectional)
            .iter()
            .map(|e| (e.get_name().to_string(), e.get_status().clone()))
            .collect::<Vec<(String, BookCopyMoveStatus)>>();
        assert_eq!(
            results_of_copy,
            [
                (String::from("file_three.txt"), BookCopyMoveStatus::Copied),
                (String::from("file_four.txt"), BookCopyMoveStatus::Copied),
            ]
        );

        let (from_local, from_foreign) = uper.cross_diff(uper.scan_area());

        let ixer_res: Vec<_> = from_local.iter().cloned().collect();
        assert_eq!(ixer_res, []);

        let ixer_res: Vec<_> = from_foreign.iter().cloned().collect();
        assert_eq!(ixer_res, []);
    }
}
