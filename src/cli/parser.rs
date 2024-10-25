use clap::Parser;
use crate::modes::ContentManager;
use crate::modes::dictionary::Dictionary;
use crate::modes::hasher::Hasher;
use crate::modes::passwords::Passwords;
use crate::modes::retriver::Retriver;

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

        let mut passwords: Option<Passwords> = None;
        if let Some(ref passwords_path) = args.passwords {
            let mut loaded_passwords = Passwords::new(passwords_path.clone());
            loaded_passwords.load();
            loaded_passwords.display();
            passwords = Some(loaded_passwords);
        }

        let mut hasher: Option<Hasher> = None;
        if let Some(ref dictionary_path) = args.dictionary {
            let mut dictionary = Dictionary::new(dictionary_path.clone());
            dictionary.load();

            if args.hash.is_none() {
                dictionary.display();
            } else if let Some(ref hash_function) = args.hash {
                let mut created_hasher = Hasher::new(dictionary, hash_function.to_string());
                created_hasher.load();
                created_hasher.display();
                hasher = Some(created_hasher);
            }
        }

        if let (Some(hasher), Some(passwords)) = (hasher, passwords) {
            let retriver = Retriver::new(hasher, passwords);
            retriver.run();
        }
    }
}

