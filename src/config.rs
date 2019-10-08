use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub struct Config {
    path: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigStorage {
    source: PathBuf,
    destination: PathBuf,
}

impl Config {
    pub fn new(path: PathBuf) -> Self {
        Config { path }
    }

    pub fn parse(&self) -> Result<(PathBuf, PathBuf), String> {
        if !self.path.exists() {
            return Err(format!(
                "Script: {} doesn't exist",
                self.path.to_str().unwrap()
            ));
        }

        let mut f = File::open(self.path.clone())
            .or_else(|e| Err(format!("fail to open config with error: {}", e)))?;

        let mut s = String::new();
        f.read_to_string(&mut s)
            .or_else(|e| Err(format!("fail to read config with error: {}", e)))?;

        let ConfigStorage {
            source,
            destination,
        } = serde_yaml::from_str(&s)
            .or_else(|e| Err(format!("fail to parse config with error: {}", e)))?;

        Ok((source, destination))
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::Config;

    #[test]
    fn parse() {
        let cfg = Config::new(PathBuf::from("tests/config/test_config.yaml"));

        let (source, destination) = cfg.parse().unwrap();

        assert_eq!(source, PathBuf::from("/test/path/to_source"));
        assert_eq!(destination, PathBuf::from("/test/path/to_destination"));
    }
}
