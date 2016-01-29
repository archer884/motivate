use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use toml;

#[derive(Debug, RustcDecodable)]
pub struct Source {
    name: String,
    quotes: Vec<String>,
}

impl Source {
    pub fn len(&self) -> usize {
        self.quotes.len()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Copy, Clone)]
pub struct Quote<'s> {
    source: &'s Source,
    idx: usize,
}

impl<'s> Quote<'s> {
    pub fn new(source: &'s Source, idx: usize) -> Quote<'s> {
        Quote {
            source: source,
            idx: idx,
        }
    }
}

impl<'s> fmt::Display for Quote<'s> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n\t\t- {}", self.source.quotes[self.idx], self.source.name)
    }
}

pub fn load<P: AsRef<Path>>(path: P) -> Result<Vec<Source>, String> {
    match File::open(path.as_ref()) {
        Err(_) => Err(format!("Unable to open file: {:?}", path.as_ref())),
        Ok(mut file) => {
            let data = {
                let mut buf = String::new();
                file.read_to_string(&mut buf).ok();
                buf
            };

            match data.parse::<toml::Value>() {
                Err(e) => Err(format!("{:?}", e)),
                Ok(value) => match value.lookup("sources") {
                    None => Err("Element not found".to_owned()),
                    Some(value) => toml::decode(value.clone()).ok_or(
                        "Unable to decode".to_owned()
                    )
                }
            }
        }
    }
}
