mod book;
mod bookshelf;
use crate::book::Book;

fn main() {
    println!("Sync your e-book library");
    let book = Book::new(String::from("My Book"));
    println!("Book name is: {}", book);
}
