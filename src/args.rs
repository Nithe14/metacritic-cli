use clap::Parser;
use std::ops::RangeInclusive;

const ALL: &str = "all";
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

    /// Specify item type (movie,
    /// game, album, tv, person, video, company, story, all)
    #[arg(short = 't', long = "type", value_parser = type_parser, default_value_t = ALL.to_owned())]
    pub itype: String,
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

fn type_parser(s: &str) -> Result<String, String> {
    let t: String = s
        .parse()
        .map_err(|_| format!("`{}` isn't a valid type", s))?;
    if s == "game"
        || s == "movie"
        || s == "album"
        || s == "tv"
        || s == "person"
        || s == "video"
        || s == "company"
        || s == "story"
        || s == "all"
    {
        Ok(t as String)
    } else {
        Err(format!("`{}` isn't a valid type! Valid types: movie, game, album, tv, person, video, company, story, all", s))
    }
}
