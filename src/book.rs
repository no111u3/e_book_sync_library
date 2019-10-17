//! Book entity
//!
//! Book representation with internal helpers

use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Eq, Clone)]
pub struct Book {
    name: String,
    path: PathBuf,
}

impl Book {
    pub fn new(name: String) -> Self {
        let path = name.clone();
        Book {
            name,
            path: PathBuf::from(path),
        }
    }

    pub fn from(path: PathBuf) -> Self {
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        Book { name, path }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn exists(&self) -> bool {
        self.path.exists()
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

use std::cmp::Ordering;

impl PartialOrd for Book {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Book {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

#[cfg(test)]
mod tests {
    use super::Book;
    use std::path::PathBuf;

    #[test]
    fn path_name_correct() {
        let book = Book::from(PathBuf::from("/local/test_book.txt"));
        assert_eq!(book.get_name(), &String::from("test_book.txt"));
        assert_eq!(book.get_path(), &PathBuf::from("/local/test_book.txt"));
    }

    #[test]
    fn correct_display() {
        let book = Book {
            name: "Test Book".to_string(),
            path: PathBuf::new(),
        };
        assert_eq!(format!("{}", book), String::from("Test Book"));
    }

    #[test]
    fn equal_name_equal_books() {
        assert_eq!(
            Book::new("Test Eq".to_string()),
            Book::new("Test Eq".to_string())
        );

        assert_eq!(
            Book::from(PathBuf::from("/local/test_book.txt")),
            Book::from(PathBuf::from("/foreign/test_book.txt"))
        );
    }

    #[test]
    fn existing_checks() {
        assert_eq!(
            Book::from(PathBuf::from("tests/iterate/file_one.txt")).exists(),
            true
        );

        assert_eq!(
            Book::from(PathBuf::from("tests/iterate/file_four.txt")).exists(),
            false
        );
    }
}
