use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use yaml_rust::YamlLoader;

pub struct Config {
    path: PathBuf,
}

impl Config {
    pub fn new(path: PathBuf) -> Self {
        Config { path }
    }

    pub fn parse(&self) -> (PathBuf, PathBuf) {
        let mut f = File::open(self.path.clone()).unwrap();

        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();

        let doc = &YamlLoader::load_from_str(&s).unwrap()[0];

        (
            PathBuf::from(doc["source"].as_str().unwrap()),
            PathBuf::from(doc["destination"].as_str().unwrap()),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::Config;

    #[test]
    fn parse() {
        let cfg = Config::new(PathBuf::from("tests/config/test_config.yaml"));

        let (source, destination) = cfg.parse();

        assert_eq!(source, PathBuf::from("/test/path/to_source"));
        assert_eq!(destination, PathBuf::from("/test/path/to_destination"));
    }
}
