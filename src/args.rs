use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Item name to search
    pub name: String,

    /// Display only the first item from the result page
    #[arg(short, long)]
    pub single: bool,
}
