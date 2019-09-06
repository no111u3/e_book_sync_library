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

    pub fn difference(&self, other: &Self) -> Self {
        Bookshelf {
            books: self.books.difference(&other.books).cloned().collect()
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
    }
}
