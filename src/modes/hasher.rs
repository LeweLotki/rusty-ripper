use sha2::{Sha256, Sha512, Digest};
use md5::Md5;
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
        let tokens: Vec<String> = self.dictionary.tokens.iter().cloned().collect();
        match self.hash_function.as_str() {
            "sha256" => {
                for token in tokens {
                    let hasher = Sha256::new();
                    self.add_hash(token.to_string(), hasher);
                }
            }
            "sha512" => {
                for token in tokens {
                    let hasher = Sha512::new();
                    self.add_hash(token.to_string(), hasher);
                }
            }
            "md5" => {
                for token in tokens {
                    let hasher = Md5::new();
                    self.add_hash(token.to_string(), hasher);
                }
            }
            _ => {
                println!("Unsupported hash function: {}", self.hash_function);
            }
        }
    }

    fn add_hash<H: Digest>(&mut self, token: String, mut hasher: H) {
        hasher.update(token.as_bytes());
        let result = hasher.finalize();
        self.tokens.push(token.clone());
        self.hashes.push(hex::encode(result));
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
