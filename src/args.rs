use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Display only the first item from the result page
    #[arg(short, long)]
    pub single: String,

    /// Display all items from the first result page
    #[arg(short, long)]
    pub all: String,
}
