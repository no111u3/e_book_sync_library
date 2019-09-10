use crate::bookshelf::Bookshelf;
use crate::indexer::Indexer;

pub struct Updater {
    local: String,
    foreign: String,
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

    pub fn update(&self) {
        let (from_local, from_foreign) = self.cross_diff(self.scan_area());
    }
}

#[cfg(test)]
mod tests {
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
}
