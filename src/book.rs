use std::fmt;

#[derive(Debug, Eq, Clone)]
pub struct Book {
    name: String
}

impl Book {
    pub fn new(name: String) -> Self {
        Book { name }
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

    #[test]
    fn correct_display() {
        let book = Book { name: "Test Book".to_string() };
        assert_eq!(format!("{}", book), String::from("Test Book"));
    }

    #[test]
    fn equal_name_equal_books() {
        assert_eq!(
            Book::new("Test Eq".to_string()),
            Book::new("Test Eq".to_string())
        );
    }
}
