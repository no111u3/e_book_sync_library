use walkdir::WalkDir;

use crate::bookshelf::Bookshelf;
use crate::book::Book;

pub struct Indexer {
    path: String
}

impl Indexer {
    pub fn new(path: String) -> Self {
        Indexer { path }
    }

    pub fn index(&self) -> Bookshelf {
        WalkDir::new(self.path.clone())
            .into_iter().filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .fold(Bookshelf::new(), |mut bs, entry| {
                bs.add(
                    Book::new(entry.path()
                        .file_name().unwrap()
                        .to_str().unwrap()
                        .to_string())
                );
                bs
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Indexer;

    #[test]
    fn iterate() {
        use crate::book::Book;

        let ixer = Indexer::new("tests/iterate".to_string());
        let ixer_res: Vec<_> = ixer.index().iter().cloned().collect();
        assert_eq!(ixer_res, [
            Book::new(String::from("file_one.txt")),
            Book::new(String::from("file_three.txt")),
            Book::new(String::from("file_two.txt")),
        ]);
    }
}
