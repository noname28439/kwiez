use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct ProfanityFilter {
    words: Vec<String>,
}

impl ProfanityFilter {
    pub fn empty() -> Self {
        ProfanityFilter { words: Vec::new()}
    }

    pub fn from_file(file: File) -> Self {
        let reader = BufReader::new(file);
        let words = reader
            .lines()
            .map(|s| s.expect("Could not read line"))
            .map(|s| Self::make_comparable(&s))
            .collect();
        ProfanityFilter { words }
    }

    fn make_comparable(s:&String) -> String {
        s.trim().to_lowercase()
    }

    pub fn check(&self, s: &String) -> bool {
        self.words.iter().any(|w| Self::make_comparable(s).contains(w))
    }
}

