//! Bookshelf entity
//!
//! Collect books and other stuff

use std::collections::BTreeSet;
use std::path::PathBuf;

use crate::book::Book;

type Books = BTreeSet<Book>;

#[derive(Debug)]
pub struct Bookshelf {
    books: Books,
    path: PathBuf,
}

impl Bookshelf {
    pub fn new() -> Self {
        Bookshelf {
            books: BTreeSet::new(),
            path: PathBuf::new(),
        }
    }

    pub fn from(path: PathBuf) -> Self {
        Bookshelf {
            books: BTreeSet::new(),
            path,
        }
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn add(&mut self, book: Book) -> bool {
        self.books.insert(book)
    }

    pub fn have(&self, book: &Book) -> bool {
        self.books.contains(book)
    }

    pub fn difference(&self, other: &Self) -> Self {
        Bookshelf {
            books: self.books.difference(&other.books).cloned().collect(),
            path: self.path.clone(),
        }
    }

    pub fn intersection(&self, other: &Self) -> Self {
        Bookshelf {
            books: self
                .books
                .intersection(&other.books)
                // Mapping from self because intersection returns items from both,
                // but we need items from self
                .map(|x| self.books.get(x).unwrap())
                .cloned()
                .collect(),
            path: self.path.clone(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Book> {
        self.books.iter()
    }
}

impl PartialEq for Bookshelf {
    fn eq(&self, other: &Self) -> bool {
        self.books == other.books
    }
}

#[cfg(test)]
mod tests {
    use super::Bookshelf;
    use crate::book::Book;
    use std::path::PathBuf;

    #[test]
    fn create() {
        use std::collections::BTreeSet;

        let bs1 = Bookshelf::new();
        let bs2 = Bookshelf {
            books: BTreeSet::new(),
            path: PathBuf::new(),
        };
        assert_eq!(bs1, bs2);

        let bs3 = Bookshelf::from(PathBuf::from("some/test/path"));
        assert_eq!(bs2, bs3);
    }

    #[test]
    fn correct_path() {
        let bs = Bookshelf::from(PathBuf::from("some/test/path"));
        assert_eq!(bs.get_path(), &PathBuf::from("some/test/path"));
    }

    #[test]
    fn add_book() {
        let book = Book::new(String::from("Test book"));
        let mut bs = Bookshelf::new();
        bs.add(book.clone());
        assert_eq!(bs.have(&book), true);
    }

    #[test]
    fn compare_bookshelf() {
        let mut bs1 = Bookshelf::new();
        bs1.add(Book::new(String::from("Test book1")));
        bs1.add(Book::new(String::from("Test book2")));
        let mut bs2 = Bookshelf::new();
        bs2.add(Book::new(String::from("Test book2")));
        bs2.add(Book::new(String::from("Test book3")));

        let diff1_to_2: Vec<_> = bs1.difference(&bs2).iter().cloned().collect();
        assert_eq!(diff1_to_2, [Book::new(String::from("Test book1"))]);

        let diff2_to_1: Vec<_> = bs2.difference(&bs1).iter().cloned().collect();
        assert_eq!(diff2_to_1, [Book::new(String::from("Test book3"))]);

        let inter1_to_2: Vec<_> = bs1.intersection(&bs2).iter().cloned().collect();
        assert_eq!(inter1_to_2, [Book::new(String::from("Test book2"))]);

        let inter2_to_1: Vec<_> = bs2.intersection(&bs1).iter().cloned().collect();
        assert_eq!(inter2_to_1, [Book::new(String::from("Test book2"))]);
    }
}
