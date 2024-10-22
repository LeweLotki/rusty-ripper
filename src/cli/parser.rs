use clap::Parser;
use crate::modes::ContentManager;
use crate::modes::dictionary::Dictionary;
use crate::modes::hasher::Hasher;
use crate::modes::passwords::Passwords;

#[derive(Parser)]
pub struct CLI {
    #[arg(short, long)]
    pub dictionary: Option<String>, 

    #[arg(long)]
    pub hash: Option<String>,

    #[arg(short, long)]
    pub passwords: Option<String>,
}

impl CLI {
    pub fn run_parser() {
        let args = Self::parse();

        if args.hash.is_some() && args.dictionary.is_none() {
            println!("Warning: Please provide a dictionary file using --dictionary.");
            return;
        }

        if let Some(ref passwords_path) = args.passwords {
            let mut passwords: Passwords = Passwords::new(passwords_path.clone());
            passwords.load();
            passwords.display();
        }

        if let Some(ref dictionary_path) = args.dictionary {
            let mut dictionary: Dictionary = Dictionary::new(dictionary_path.clone());
            dictionary.load();

            if args.hash.is_none() {
                dictionary.display();
            } else {
                if let Some(ref hash_function) = args.hash {
                    let mut hasher: Hasher = Hasher::new(dictionary, hash_function.to_string());
                    hasher.load();
                    hasher.display();
                }
            }
        }
    }
}

