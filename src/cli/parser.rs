use clap::Parser;
use crate::modes::{dictionary, hashes};

#[derive(Parser)]
pub struct CLI {
    #[arg(short, long)]
    pub dictionary: Option<String>, 

    #[arg(long)]
    pub hash: Option<String>,
}

impl CLI {
    pub fn run_parser() {
        let args = Self::parse();

        if args.hash.is_some() && args.dictionary.is_none() {
            println!("Warning: Please provide a dictionary file using --dictionary.");
            return;
        }

        if let Some(ref dictionary_path) = args.dictionary {
            let mut dictionary = dictionary::Dictionary::new(dictionary_path.clone());
            dictionary::load(&mut dictionary);

            if args.hash.is_none() {
                dictionary::display(&mut dictionary);
            }

            if let Some(ref hash_function) = args.hash {
                hashes::display(&dictionary, hash_function);
            }
        }
    }
}

