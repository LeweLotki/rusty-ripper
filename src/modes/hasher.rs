use sha2::{Sha256, Sha512, Digest};
use md5::Md5;
use hex;

use crate::modes::dictionary::Dictionary;
use crate::modes::ContentManager;

use rayon::prelude::*;

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
            hash_function,
        }
    }

    pub fn load_hashes(&mut self) {
        let (tokens, hashes) = {
            let tokens_ref = &self.dictionary.tokens;

            match self.hash_function.as_str() {
                "sha256" => {
                    self.hash_tokens_in_parallel::<Sha256>(tokens_ref)
                }
                "sha512" => {
                    self.hash_tokens_in_parallel::<Sha512>(tokens_ref)
                }
                "md5" => {
                    self.hash_tokens_in_parallel::<Md5>(tokens_ref)
                }
                _ => {
                    println!("Unsupported hash function: {}", self.hash_function);
                    (Vec::new(), Vec::new())
                }
            }
        };

        self.tokens = tokens;
        self.hashes = hashes;
    }

    fn hash_tokens_in_parallel<H>(&self, tokens: &Vec<String>) -> (Vec<String>, Vec<String>)
    where
        H: Digest + Send + Sync + 'static,
    {
        tokens
            .par_iter()
            .map(|token| {
                let hasher = H::new();
                let hash = compute_hash(token, hasher);
                (token.clone(), hash)
            })
            .unzip()
    }

    pub fn display_hashes(&self) {
        for (token, hash) in self.tokens.iter().zip(self.hashes.iter()) {
            println!("{} => {}", token, hash);
        }
    }
}

fn compute_hash<H: Digest>(token: &str, mut hasher: H) -> String {
    hasher.update(token.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

impl ContentManager for Hasher {
    fn load(&mut self) {
        self.load_hashes();
    }

    fn display(&self) {
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

