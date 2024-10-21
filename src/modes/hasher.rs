use sha2::{Sha256, Digest}; 
use hex;

use crate::modes::dictionary::Dictionary;
use crate::modes::ContentManager;

pub struct Hasher {
    pub dictionary: Dictionary,
    pub hashes: Vec<String>,
    pub tokens: Vec<String>,
    pub hash_function: String,
}

impl Hasher {
   pub fn new(dictionary: Dictionary, hash_function: String) -> Self {
        Self {
            dictionary,
            hashes: Vec::new(),
            tokens: Vec::new(),
            hash_function
        }
    }

    pub fn load_hashes(&mut self) -> () {
        match self.hash_function.as_str() {
            "sha256" => {
                for token in &self.dictionary.tokens {
                    let mut hasher = Sha256::new();
                    hasher.update(token.as_bytes());
                    let result = hasher.finalize();
                    self.tokens.push(token.clone());
                    self.hashes.push(hex::encode(result));
                }
            }
            _ => {
                println!("Unsupported hash function: {}", self.hash_function);
            }
        }
    }

    pub fn display_hashes(&self) {
        for (token, hash) in self.tokens.iter().zip(self.hashes.iter()) {
            println!("{} -> {}", token, hash);
        }
    }
}

impl ContentManager for Hasher {
    fn load(&mut self) -> () {
       self.load_hashes(); 
    }

    fn display(&self) -> () {
        self.display_hashes();
    }
}
