use clap::Parser;
use std::ops::RangeInclusive;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Item name to search.
    pub name: String,

    /// Print only the first item from the result page. Works as -n 1.
    #[arg(short, long)]
    pub single: bool,

    /// Print only n firsts items from the result page (1-10).
    #[arg(short, long, value_parser = n_in_range, default_value_t = 3)]
    pub number_of_results: usize,
}

const N_RANGE: RangeInclusive<usize> = 1..=10;

fn n_in_range(s: &str) -> Result<usize, String> {
    let n: usize = s.parse().map_err(|_| format!("`{}` isn't a number", s))?;
    if N_RANGE.contains(&n) {
        Ok(n as usize)
    } else {
        Err(format!(
            "Number not in range {}-{}",
            N_RANGE.start(),
            N_RANGE.end()
        ))
    }
}
