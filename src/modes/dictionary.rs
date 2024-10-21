use std::fs;
use std::io::{self, Read};

pub struct Dictionary {
    pub path: String,
    pub content: String,
}

impl Dictionary {
    pub fn new(path: String) -> Dictionary {
        Dictionary {
            path,
            content: String::new(),
        }
    }

    pub fn load_content(&mut self) -> io::Result<()> {
        let mut file = fs::File::open(&self.path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        self.content = content;
        Ok(())
    }

    pub fn display_content(&self) {
        println!("File content:\n{}", self.content);
    }
}

pub fn display(path: &String) {
    let mut dictionary = Dictionary::new(path.clone());

    if dictionary.load_content().is_ok() {
        dictionary.display_content();
    } else {
        println!("Failed to load file: {}", path);
    }
}

