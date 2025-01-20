use hex;
use md5::Md5;
use sha2::{Digest, Sha256, Sha512};

use crate::modes::dictionary::Dictionary;
use crate::modes::ContentManager;

use rayon::prelude::*;

#[derive(Debug)]
pub enum HashFunction {
    Sha256,
    Sha512,
    Md5,
}

impl HashFunction {
    pub fn from_str(name: &str) -> Option<HashFunction> {
        match name.to_lowercase().as_str() {
            "sha256" => Some(HashFunction::Sha256),
            "sha512" => Some(HashFunction::Sha512),
            "md5" => Some(HashFunction::Md5),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Hasher {
    pub dictionary: Dictionary,
    pub hashes: Vec<String>,
    pub tokens: Vec<String>,
    pub hash_function: HashFunction,
    pub salt: String,
}

impl Hasher {
    pub fn new(dictionary: Dictionary, hash_function: HashFunction, salt: String) -> Self {
        let mut hasher: Self = Self {
            dictionary,
            hashes: Vec::new(),
            tokens: Vec::new(),
            hash_function,
            salt,
        };
        hasher.load_hashes();
        hasher
    }

    pub fn load_hashes(&mut self) {
        let (tokens, hashes) = {
            let tokens_ref = &self.dictionary.tokens;
            let salt_ref = &self.salt;

            match self.hash_function {
                HashFunction::Sha256 => hash_tokens_in_parallel::<Sha256>(tokens_ref, salt_ref),
                HashFunction::Sha512 => hash_tokens_in_parallel::<Sha512>(tokens_ref, salt_ref),
                HashFunction::Md5 => hash_tokens_in_parallel::<Md5>(tokens_ref, salt_ref),
            }
        };

        self.tokens = tokens.into_iter().map(|s| s.to_owned()).collect();
        self.hashes = hashes;
    }
}

fn hash_tokens_in_parallel<'a, H>(
    tokens: &'a Vec<String>,
    salt: &str,
) -> (Vec<&'a str>, Vec<String>)
where
    H: Digest + Send + Sync + 'static,
{
    tokens
        .par_iter()
        .map(|token| {
            let hasher = H::new();
            let hash = compute_hash(token, hasher, salt);
            (token.as_str(), hash)
        })
        .unzip()
}

fn compute_hash<H: Digest>(token: &str, mut hasher: H, salt: &str) -> String {
    let token_salt = format!("{}{}", token, salt);
    hasher.update(token_salt.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

impl ContentManager for Hasher {
    fn display(&self) {
        match self.hash_function {
            HashFunction::Sha256 => {
                println!("Hash Function: SHA-256");
                println!("Description: SHA-256 is part of the SHA-2 family and produces a 256-bit hash value.");
            }
            HashFunction::Sha512 => {
                println!("Hash Function: SHA-512");
                println!("Description: SHA-512 is part of the SHA-2 family and produces a 512-bit hash value.");
            }
            HashFunction::Md5 => {
                println!("Hash Function: MD5");
                println!("Description: MD5 is an older hashing algorithm that produces a 128-bit hash value.");
                println!("Note: MD5 is considered cryptographically broken and unsuitable for further use.");
            }
        }
    }
}
