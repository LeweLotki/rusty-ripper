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
            println!("{} => {}", token, hash);
        }
    }
}

impl ContentManager for Hasher {
    fn load(&mut self) -> () {
       self.load_hashes(); 
    }

    fn display(&self) -> () {
        match self.hash_function.as_str() {
            "sha256" => {
                println!("Hash Function: SHA-256");
                println!("Description: SHA-256 is part of the SHA-2 family and produces a 256-bit hash value.");
            }
            "sha512" => {
                println!("Hash Function: SHA-512");
                println!("Description: SHA-512 is part of the SHA-2 family and produces a 512-bit hash value.");
            }
            "md5" => {
                println!("Hash Function: MD5");
                println!("Description: MD5 is an older hashing algorithm that produces a 128-bit hash value.");
                println!("Note: MD5 is considered cryptographically broken and unsuitable for further use.");
            }
            _ => {
                println!("Unsupported hash function: {}", self.hash_function);
            }
        }
    }
}
