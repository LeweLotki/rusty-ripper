use sha2::{Sha256, Digest}; 
use crate::modes::dictionary::Dictionary;

pub fn hash_tokens(dictionary: &Dictionary, hash_function: &str) {
    match hash_function {
        "sha256" => {
            println!("Hashing tokens with SHA-256:");
            for token in &dictionary.tokens {
                let mut hasher = Sha256::new();
                hasher.update(token.as_bytes());
                let result = hasher.finalize();
                println!("{} -> {:x}", token, result);
            }
        }
        _ => {
            println!("Unsupported hash function: {}", hash_function);
        }
    }
}

pub fn display(dictionary: &Dictionary, hash_function: &str) {
    hash_tokens(dictionary, hash_function);
}
