use std::fmt;

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

#[cfg(test)]
mod tests {
    use super::Book;

    #[test]
    fn correct_display() {
        let book = Book { name: "Test Book".to_string() };
        assert_eq!(format!("{}", book), String::from("Test Book"));
    }
}
