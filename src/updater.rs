use crate::bookshelf::Bookshelf;
use crate::indexer::Indexer;

pub struct Updater {
    local: String,
    foreign: String
}

impl Updater {
    pub fn new(local: String, foreign: String) -> Self {
        Updater {
            local,
            foreign
        }
    }

    fn scan_area(&self) -> (Bookshelf, Bookshelf) {
        (
            Indexer::new(self.local.clone()).index(),
            Indexer::new(self.foreign.clone()).index()
        )
    }

    pub fn update(&self) {
    }
}

#[cfg(test)]
mod tests {
    use super::Updater;

    #[test]
    fn scan_area() {
        use crate::book::Book;
    
        let uper = Updater::new(
            "tests/scan_area/local".to_string(),
            "tests/scan_area/foreign".to_string()
        );

        let scan_a = uper.scan_area();

        let ixer_res: Vec<_> = scan_a.0.iter().cloned().collect();
        assert_eq!(ixer_res, [
            Book::new(String::from("file_one.txt")),
            Book::new(String::from("file_three.txt")),
            Book::new(String::from("file_two.txt")),
        ]);

        let ixer_res: Vec<_> = scan_a.1.iter().cloned().collect();
        assert_eq!(ixer_res, [
            Book::new(String::from("file_four.txt")),
            Book::new(String::from("file_one.txt")),
            Book::new(String::from("file_two.txt")),
        ]);
    }
}
