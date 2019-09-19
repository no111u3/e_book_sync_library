use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;

use crate::bookshelf::Bookshelf;
use crate::indexer::Indexer;

pub struct Updater {
    local: PathBuf,
    foreign: PathBuf,
}

pub enum Update {
    OnlyFromLocal,
    OnlyFromLocalSync,
    OnlyFromForeign,
    OnlyFromForeignSync,
    Bidirectional,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BookTransferStatus {
    Copied,
    Moved,
    Error(String),
}

impl fmt::Display for BookTransferStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BookTransferStatus::Copied => String::from("Copied"),
                BookTransferStatus::Moved => String::from("Moved"),
                BookTransferStatus::Error(e) => format!("Move error: {}", e),
            }
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BookStatus {
    name: String,
    src: PathBuf,
    dst: PathBuf,
    status: BookTransferStatus,
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

    pub fn get_status(&self) -> &BookTransferStatus {
        &self.status
    }
}

fn cross_diff((local, foreign): (Bookshelf, Bookshelf)) -> (Bookshelf, Bookshelf) {
    (local.difference(&foreign), foreign.difference(&local))
}

fn create_dir_for_path(path: &PathBuf) -> io::Result<()> {
    let path_dir = path.parent().unwrap();

    if !path_dir.exists() {
        fs::create_dir(path_dir)
    } else {
        Ok(())
    }
}

fn copy_files(books: Bookshelf, destination: &PathBuf) -> Vec<BookStatus> {
    books
        .iter()
        .map(|b| {
            let mut dest_path = destination.to_path_buf();
            dest_path.push(b.get_path().strip_prefix(books.get_path()).unwrap());

            let status = create_dir_for_path(&dest_path);

            BookStatus {
                name: b.get_name().to_string(),
                src: b.get_path().to_path_buf(),
                dst: dest_path.to_path_buf(),
                status: if let Ok(_) = status {
                    match fs::copy(b.get_path(), dest_path) {
                        Err(e) => BookTransferStatus::Error(e.to_string()),
                        Ok(_) => BookTransferStatus::Copied,
                    }
                } else {
                    BookTransferStatus::Error(status.err().unwrap().to_string())
                },
            }
        })
        .collect()
}

fn move_files(books_src: Bookshelf, books_dst: Bookshelf) -> Vec<BookStatus> {
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

            let status = create_dir_for_path(&dest_path);

            BookStatus {
                name: book_src.get_name().to_string(),
                src: book_dst.get_path().to_path_buf(),
                dst: dest_path.to_path_buf(),
                status: {
                    if let Ok(_) = status {
                        match fs::rename(book_dst.get_path(), dest_path) {
                            Err(e) => BookTransferStatus::Error(e.to_string()),
                            Ok(_) => BookTransferStatus::Moved,
                        }
                    } else {
                        BookTransferStatus::Error(status.err().unwrap().to_string())
                    }
                },
            }
        })
        .collect()
}

impl Updater {
    pub fn new(local: PathBuf, foreign: PathBuf) -> Self {
        Updater {
            local: local,
            foreign: foreign,
        }
    }

    fn scan_area(&self) -> (Bookshelf, Bookshelf) {
        (
            Indexer::new(self.local.clone()).index(),
            Indexer::new(self.foreign.clone()).index(),
        )
    }

    pub fn update(&self, update: Update) -> Vec<BookStatus> {
        use Update::*;

        let (from_local, from_foreign) = match update {
            Bidirectional | OnlyFromLocal | OnlyFromForeign => cross_diff(self.scan_area()),
            OnlyFromLocalSync | OnlyFromForeignSync => self.scan_area(),
        };

        match update {
            OnlyFromLocal => copy_files(from_local, &self.foreign),
            OnlyFromLocalSync => move_files(from_local, from_foreign),
            OnlyFromForeign => copy_files(from_foreign, &self.local),
            OnlyFromForeignSync => move_files(from_foreign, from_local),
            Bidirectional => {
                let mut results_of_copy: Vec<BookStatus> = Vec::new();
                results_of_copy.append(&mut copy_files(from_local, &self.foreign));
                results_of_copy.append(&mut copy_files(from_foreign, &self.local));
                results_of_copy
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::*;
    use crate::book::Book;

    #[test]
    fn scan_area_check() {
        let uper = Updater::new(
            PathBuf::from("tests/scan_area/local"),
            PathBuf::from("tests/scan_area/foreign"),
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
    fn cross_diff_check() {
        let uper = Updater::new(
            PathBuf::from("tests/scan_area/local"),
            PathBuf::from("tests/scan_area/foreign"),
        );

        let (from_local, from_foreign) = cross_diff(uper.scan_area());

        let ixer_res: Vec<_> = from_local.iter().cloned().collect();
        assert_eq!(ixer_res, [Book::new(String::from("file_three.txt")),]);

        let ixer_res: Vec<_> = from_foreign.iter().cloned().collect();
        assert_eq!(ixer_res, [Book::new(String::from("file_four.txt")),]);
    }

    #[test]
    fn copy_files_check() {
        let uper = Updater::new(
            PathBuf::from("tests/copy_files/local"),
            PathBuf::from("tests/copy_files/foreign"),
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

        let (from_local, from_foreign) = cross_diff(uper.scan_area());

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

        let results_of_copy = copy_files(from_local, from_foreign.get_path())
            .iter()
            .map(|e| (e.get_name().to_string(), e.get_status().clone()))
            .collect::<Vec<(String, BookTransferStatus)>>();
        assert_eq!(
            results_of_copy,
            [
                (String::from("file_four.txt"), BookTransferStatus::Copied),
                (String::from("file_one.txt"), BookTransferStatus::Copied),
                (String::from("file_three.txt"), BookTransferStatus::Copied),
                (String::from("file_two.txt"), BookTransferStatus::Copied),
            ]
        );

        let (from_local, _) = cross_diff(uper.scan_area());

        let ixer_res: Vec<_> = from_local.iter().cloned().collect();
        assert_eq!(ixer_res, []);
    }

    #[test]
    fn move_files_check() {
        let uper = Updater::new(
            PathBuf::from("tests/move_files/local"),
            PathBuf::from("tests/move_files/foreign"),
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

        let results_of_move = move_files(from_local, from_foreign)
            .iter()
            .map(|e| (e.get_name().to_string(), e.get_status().clone()))
            .collect::<Vec<(String, BookTransferStatus)>>();
        assert_eq!(
            results_of_move,
            [(String::from("file_four.txt"), BookTransferStatus::Moved),]
        );
    }

    #[test]
    fn update_files_check() {
        let uper = Updater::new(
            PathBuf::from("tests/update_files/local"),
            PathBuf::from("tests/update_files/foreign"),
        );

        let delete: fn() -> std::io::Result<()> = || {
            fs::remove_file("tests/update_files/foreign/file_three.txt")?;
            fs::remove_file("tests/update_files/local/file_four.txt")?;
            Ok(())
        };

        match delete() {
            Err(e) => panic!("Delete error: {}", e),
            Ok(_) => (),
        }

        let (from_local, from_foreign) = cross_diff(uper.scan_area());

        let ixer_res: Vec<_> = from_local.iter().cloned().collect();
        assert_eq!(ixer_res, [Book::new(String::from("file_three.txt")),]);

        let ixer_res: Vec<_> = from_foreign.iter().cloned().collect();
        assert_eq!(ixer_res, [Book::new(String::from("file_four.txt")),]);

        let results_of_copy = uper
            .update(Update::Bidirectional)
            .iter()
            .map(|e| (e.get_name().to_string(), e.get_status().clone()))
            .collect::<Vec<(String, BookTransferStatus)>>();
        assert_eq!(
            results_of_copy,
            [
                (String::from("file_three.txt"), BookTransferStatus::Copied),
                (String::from("file_four.txt"), BookTransferStatus::Copied),
            ]
        );

        let (from_local, from_foreign) = cross_diff(uper.scan_area());

        let ixer_res: Vec<_> = from_local.iter().cloned().collect();
        assert_eq!(ixer_res, []);

        let ixer_res: Vec<_> = from_foreign.iter().cloned().collect();
        assert_eq!(ixer_res, []);

        match delete() {
            Err(e) => panic!("Delete error: {}", e),
            Ok(_) => (),
        }

        let mut results_of_copy_two = uper.update(Update::OnlyFromLocal);
        results_of_copy_two.append(&mut uper.update(Update::OnlyFromForeign));

        let results_of_copy_two = results_of_copy_two
            .iter()
            .map(|e| (e.get_name().to_string(), e.get_status().clone()))
            .collect::<Vec<(String, BookTransferStatus)>>();

        assert_eq!(results_of_copy, results_of_copy_two);
    }
}
