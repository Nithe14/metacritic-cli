use clap::Parser;
use std::ops::RangeInclusive;

const ALL: &str = "all";
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Word to search.
    /// You can provide "coming-soon" to get upcoming game releases.
    /// Examples:
    /// `metacritic-cli "witcher 3"`
    /// `metacritic-cli coming-soon -p ps5`
    #[arg(verbatim_doc_comment)]
    pub name: String,

    /// Print only the first object from the result page.
    /// Works as -n 1.
    ///
    #[arg(short, long, verbatim_doc_comment)]
    pub single: bool,

    /// Print output as json
    ///
    #[arg(short, long, verbatim_doc_comment)]
    pub json: bool,

    /// Print only n first objects from the result page (n = 1-10).
    #[arg(short, long, value_parser = n_in_range, default_value_t = 3)]
    pub number_of_results: usize,

    /// Specify object type.
    /// Available types:
    /// movie, game, album, tv, person, video, company, story, all
    ///
    #[arg(short = 't', long = "type", value_parser = type_parser, default_value_t = ALL.to_owned(), verbatim_doc_comment)]
    pub itype: String,

    /// Specify platform (only for game type for now).
    /// Available options (ps5 and xbox-series-x is only available for "coming-soon" for now - it's because of metacritic not me, sorry):
    /// ps, ps2, ps3, ps4, ps5, xbox, xbox360, xboxone, xbox-series-x, switch, pc, ds, 3ds, ps-vita, psp, wii, wii-u, gameboy-advance, iphone, all
    ///
    #[arg(short, long, default_value_t = ALL.to_owned(), verbatim_doc_comment)]
    pub platform: String,
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
