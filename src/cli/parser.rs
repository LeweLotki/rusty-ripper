use clap::CommandFactory;
use clap::Parser;

use std::path::PathBuf;

use crate::modes::dictionary::Dictionary;
use crate::modes::hasher::{HashFunction, Hasher};
use crate::modes::passwords::Passwords;
use crate::modes::retriver::Retriver;
use crate::modes::ContentManager;

#[derive(Debug, Parser)]
pub struct CLI {
    #[arg(short, long)]
    pub dictionary: Option<PathBuf>,

    #[arg(long)]
    pub hash: Option<String>,

    #[arg(short, long)]
    pub passwords: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub generate: bool,

    #[arg(short, long)]
    pub salt: Option<String>,
}

impl CLI {
    pub fn run_parser() {
        let args = Self::parse();

        let dictionary_flag = args.dictionary.is_some();
        let hash_flag = args.hash.is_some();
        let passwords_flag = args.passwords.is_some();
        let generate_flag = args.generate;

        let dictionary_flag_val = args.dictionary.clone().unwrap_or_default();
        let hash_flag_val = args.hash.clone().unwrap_or_default();
        let passwords_flag_val = args.passwords.clone().unwrap_or_default();
        let salt_flag_val = args.salt.clone().unwrap_or_default();

        if dictionary_flag && !hash_flag && !passwords_flag {
            let dictionary = Dictionary::new(dictionary_flag_val);
            dictionary.display();
            return;
        }

        if hash_flag && !dictionary_flag && !passwords_flag {
            if let Some(hash_fn_enum) = HashFunction::from_str(hash_flag_val.as_str()) {
                let dummy_dictionary = Dictionary::new(String::new());
                let hasher = Hasher::new(dummy_dictionary, hash_fn_enum, salt_flag_val);
                hasher.display();
                return;
            } else {
                println!("Unsupported hash function: {}", hash_flag_val);
                return;
            }
        }

        if passwords_flag && !dictionary_flag && !hash_flag {
            let passwords = Passwords::new(passwords_flag_val.clone());
            passwords.display();
            return;
        }

        if dictionary_flag && hash_flag && passwords_flag {
            let dictionary = Dictionary::new(dictionary_flag_val);
            let passwords = Passwords::new(passwords_flag_val);

            if let Some(hash_fn_enum) = HashFunction::from_str(hash_flag_val.as_str()) {
                let hasher = Hasher::new(dictionary, hash_fn_enum, salt_flag_val);

                let retriver = Retriver::new(&hasher, &passwords);
                retriver.run();
                return;
            } else {
                println!("Unsupported hash function: {}", hash_flag_val);
                return;
            }
        }

        if generate_flag && dictionary_flag && hash_flag {
            let dictionary = Dictionary::new(dictionary_flag_val);
            if let Some(hash_fn_enum) = HashFunction::from_str(hash_flag_val.as_str()) {
                let hasher = Hasher::new(dictionary, hash_fn_enum, salt_flag_val);
                let hashes = &hasher.hashes;
                let tokens = &hasher.tokens;
                for (hash, token) in hashes.iter().zip(tokens.iter()) {
                    println!("{},{}", hash, token);
                }

                return;
            } else {
                println!("Unsupported hash function: {}", hash_flag_val);
                return;
            }
        }

        println!("Error: Wrong flags combination.");
        CLI::command().print_help().unwrap();
        println!();
    }
}
