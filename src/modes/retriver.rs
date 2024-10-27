use crate::modes::hasher::Hasher;
use crate::modes::passwords::Passwords;
use radix_trie::Trie; 

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
        let mut hash_trie: Trie<String, String> = Trie::new();
        for (hash, token) in self.hashes.iter().zip(self.tokens.iter()) {
            hash_trie.insert(hash.clone(), token.clone());
        }

        for (login, password_hash) in self.logins.iter().zip(self.passwords.iter()) {
            if let Some(token) = hash_trie.get(password_hash) {
                println!("Login: {}, Password: {}", login, token);
            }
        }
    }
}

