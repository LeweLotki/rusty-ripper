use std::fs::File;
use std::io::{self};
use csv::ReaderBuilder;
use crate::modes::ContentManager;

pub struct Passwords {
    pub path: String,
    pub content: String,
    pub logins: Vec<String>,
    pub passwords: Vec<String>,
}

impl Passwords {
    pub fn new(path: String) -> Self {
        Self {
            path,
            content: String::new(),
            logins: Vec::new(),
            passwords: Vec::new(),
        }
    }

    pub fn load_content(&mut self) -> io::Result<()> {
        let file = File::open(&self.path)?;
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        for result in rdr.records() {
            let record = result?;
            if record.len() == 2 {
                self.logins.push(record[0].to_string());
                self.passwords.push(record[1].to_string());
            }
        }
        Ok(())
    }

    pub fn validate(&self) -> bool {
        !self.logins.is_empty() && self.logins.len() == self.passwords.len()
    }

    pub fn display_tokens(&self) {
        for (login, password) in self.logins.iter().zip(self.passwords.iter()) {
            println!("Login: {}, Password: {}", login, password);
        }
    }
}

impl ContentManager for Passwords {
    fn load(&mut self) {
        if self.load_content().is_ok() {
            if self.validate() {
                println!("Passwords loaded successfully.");
            } else {
                println!("Error: The number of logins and passwords do not match.");
            }
        } else {
            println!("Failed to load the file.");
        }
    }

    fn display(&self) {
        self.display_tokens();
    }
}

