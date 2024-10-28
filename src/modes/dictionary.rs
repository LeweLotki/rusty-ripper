use std::fs;
use std::io::{self, Read};

use crate::modes::ContentManager;

#[derive(Debug)]
pub struct Dictionary {
    pub path: String,
    pub content: String,
    pub tokens: Vec<String>,
}

impl Dictionary {
    pub fn new(path: String) -> Self {
         Self {
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
}

impl ContentManager for Dictionary {
    fn load(&mut self) -> () {
        if self.load_content().is_ok() {
            if self.validate() {
                self.parse_tokens();  
            } else {
                println!("The file is not a valid dictionary.");
            }
        } else {
            println!("Failed to load file: {}", self.path);
        }
           
    }
    fn display(&self) -> () {
        if self.tokens.is_empty() {
            println!("The dictionary could not be loaded due to wrong formatting.");
        } else {
            println!("Dictionary loaded successfully with {} tokens.", self.tokens.len());
        }
    }
}
