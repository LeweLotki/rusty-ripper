use crate::modes::hasher::Hasher;
use crate::modes::passwords::Passwords;

use std::collections::HashMap;
use rayon::prelude::*; 

pub struct Retriver {
    pub tokens: Vec<String>,    // Tokens used to create hashes
    pub hashes: Vec<String>,    // Hashes created from tokens
    pub logins: Vec<String>,    // Logins from CSV file
    pub passwords: Vec<String>, // Hashed passwords for given logins
}

impl Retriver {
    pub fn new(hasher: Hasher, passwords: Passwords) -> Self {
        Self {
            tokens: hasher.tokens.clone(),
            hashes: hasher.hashes.clone(),
            logins: passwords.logins.clone(),
            passwords: passwords.passwords.clone(),
        }
    }

    pub fn run(&self) {
        let hash_map: HashMap<&String, &String> = self.hashes.par_iter().zip(self.tokens.par_iter()).collect();

        self.logins.par_iter().zip(self.passwords.par_iter()).for_each(|(login, password_hash)| {
            if let Some(token) = hash_map.get(password_hash) {
                println!("Login: {}, Password: {}", login, token);
            }
        });
    }
}

