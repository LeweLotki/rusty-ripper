use clap::Parser;
use crate::modes::dictionary;

#[derive(Parser)]
pub struct CLI {

    #[arg(short, long)]
    pub dictionary: Option<String>,
    
}

impl CLI {
    pub fn run_parser() -> () {
        let args = Self::parse();
        if let Some(ref dictionary_path) = args.dictionary {
            dictionary::display(&dictionary_path);
        }
    }
}

