use crate::modes::hasher::Hasher;
use crate::modes::passwords::Passwords;

pub struct Retriver {
    pub tokens: Vec<String>, // token used to create a hash    
    pub hashes: Vec<String>, // hash created from particular token
    pub logins: Vec<String>, // login of user from csv file
    pub passwords: Vec<String>, // hashed password for given login
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
        for (i, hash) in self.hashes.iter().enumerate() {
            for (j, password) in self.passwords.iter().enumerate() {
                if hash == password {
                    if let (Some(login), Some(token)) = (self.logins.get(j), self.tokens.get(i)) {
                        println!("Login: {}, Token: {}", login, token);
                    }
                }
            }
        }
    }
}

