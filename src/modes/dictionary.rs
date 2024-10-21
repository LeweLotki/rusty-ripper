use std::fs;
use std::io::{self, Read};

pub struct Dictionary {
    pub path: String,
    pub content: String,
    pub tokens: Vec<String>,
}

impl Dictionary {
    pub fn new(path: String) -> Dictionary {
        Dictionary {
            path,
            content: String::new(),
            tokens: Vec::new(),
        }
    }

    pub fn load_content(&mut self) -> io::Result<()> {
        let mut file = fs::File::open(&self.path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        self.content = content;
        Ok(())
    }

    pub fn validate(&self) -> bool {
        self.content.lines().all(|line| line.split_whitespace().count() == 1)
    }

    pub fn parse_tokens(&mut self) {
        self.tokens = self.content.lines().map(|line| line.to_string()).collect();
    }

    pub fn display_tokens(&self) {
        println!("Tokens in dictionary:");
        for token in &self.tokens {
            println!("{}", token);
        }
    }
}

pub fn display(path: &String) {
    let mut dictionary = Dictionary::new(path.clone());

    if dictionary.load_content().is_ok() {
        if dictionary.validate() {
            dictionary.parse_tokens();  
            dictionary.display_tokens();
        } else {
            println!("The file is not a valid dictionary.");
        }
    } else {
        println!("Failed to load file: {}", path);
    }
}

