use clap::Parser;

#[derive(Parser)]
pub struct CLI {
    #[arg(long)]
    pub name: String,
    #[arg(short, long)]
    pub loud: bool
}

impl CLI {
    pub fn run_parser() -> () {
        let args = Self::parse();

        if args.loud {
            println!("HELLO {}", args.name.to_uppercase());
        } else {
            println!("Hello {}", args.name);
        }
    }
}

