use clap::Parser;
use crate::modes::greet;

#[derive(Parser)]
pub struct CLI {

    #[arg(long)]
    pub name: Option<String>,
    
    #[arg(short, long)]
    pub loud: bool
}

impl CLI {
    pub fn run_parser() -> () {
        let args = Self::parse();
        if let Some(ref name) = args.name {
            greet::greet(&name, args.loud);
        }
    }
}

