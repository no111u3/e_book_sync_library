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

    pub fn update(&self) {
    }
}

#[cfg(test)]
mod tests {
    use super::Updater;

    #[test]
    fn scan_area() {
        let uper = Updater::new(
            "tests/scan_area/local".to_string(),
            "tests/scan_area/foreign".to_string()
        );
    }
}
