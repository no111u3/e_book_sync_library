use std::collections::BTreeSet;

use crate::book::Book;

type Books = BTreeSet<Book>;

#[derive(Debug)]
pub struct Bookshelf {
    books: Books,
}

impl Bookshelf {
    pub fn new() -> Self {
        Bookshelf {
            books: BTreeSet::new()
        }
    }

    pub fn add(&mut self, book: Book) -> bool {
        self.books.insert(book)
    }

    pub fn have(&self, book: &Book) -> bool {
        self.books.contains(book)
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

    #[test]
    fn create() {
        use std::collections::BTreeSet;
    
        let bs1 = Bookshelf::new();
        let bs2 = Bookshelf { books: BTreeSet::new() };
        assert_eq!(bs1, bs2);
    }

    #[test]
    fn add_book() {
        let book = Book::new(String::from("Test book")); 
        let mut bs = Bookshelf::new();
        bs.add(book.clone());
        assert_eq!(bs.have(&book), true);
    }
}
