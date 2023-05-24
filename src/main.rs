use std::error::Error;
use clap::Parser;
use namedrop::NameDrop;

/// A name dropper -- generates random names in <adjective noun> format
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of names to drop
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let name_drop = NameDrop::new();
    name_drop.run(args.count)
}
