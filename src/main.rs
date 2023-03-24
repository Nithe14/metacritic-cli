mod args;
mod metacriticresults;

use args::Args;
use clap::Parser;
use colored::Colorize;
use metacriticresults::{MetacriticResult, TSP};
use reqwest::blocking::{RequestBuilder, Response};
use serde_json::to_string;
use urlencoding::encode;

#[macro_use]
extern crate serde_derive;

fn main() {
    let args = Args::parse();
    let mut_number_of_results: usize;

    let url = set_url(&args.name, &args.platform, &args.itype);

    let response = make_request(url).unwrap();

    if args.single {
        mut_number_of_results = 1;
    } else {
        mut_number_of_results = args.number_of_results;
    }

    let document = scraper::Html::parse_document(&response);

    let final_results = scrap(&document, mut_number_of_results, args.name); //scraping HTML data

    if args.json {
        println!("{}", to_string(&final_results).unwrap());
    } else {
        print_pretty(final_results);
    }
}

fn set_url(name: &String, platform: &String, itype: &String) -> String {
    let url: String;
    if name == &String::from("coming-soon") {
        url = format!(
            "https://www.metacritic.com/browse/games/release-date/coming-soon/{}/date",
            platform,
        );
    } else {
        let search_args;
        match platform.as_str() {
            "ps4" => search_args = String::from("?plats[72496]=1&search_type=advanced"),
            "ps3" => search_args = String::from("?search_type=advanced&plats[1]=1"),
            "xbox-one" => search_args = String::from("?search_type=advanced&plats[80000]=1"),
            "switch" => search_args = String::from("?plats[268409]=1&search_type=advanced"),
            "xbox-360" => search_args = String::from("?search_type=advanced&plats[2]=1"),
            "pc" => search_args = String::from("?search_type=advanced&plats[3]=1"),
            "ds" => search_args = String::from("?search_type=advanced&plats[4]=1"),
            "3ds" => search_args = String::from("?search_type=advanced&plats[16]=1"),
            "ps-vita" => search_args = String::from("?search_type=advanced&plats[67365]=1"),
            "psp" => search_args = String::from("?search_type=advanced&plats[7]=1"),
            "wii" => search_args = String::from("?search_type=advanced&plats[8]=1"),
            "wii-u" => search_args = String::from("?search_type=advanced&plats[68410]=1"),
            "ps2" => search_args = String::from("?search_type=advanced&plats[6]=1"),
            "ps" => search_args = String::from("?search_type=advanced&plats[10]=1"),
            "gameboy-advance" => search_args = String::from("?search_type=advanced&plats[11]=1"),
            "iphone" => search_args = String::from("?search_type=advanced&plats[9]=1"),
            "xbox" => search_args = String::from("?search_type=advanced&plats[12]=1"),
            "gamecube" => search_args = String::from("?search_type=advanced&plats[13]=1"),
            "nintendo64" => search_args = String::from("?search_type=advanced&platls[14]=1"),
            "dreamcast" => search_args = String::from("?search_type=advanced&plats[15]=1"),
            _ => search_args = String::from(""),
        }
        url = format!(
            "https://www.metacritic.com/search/{}/{}/results{}",
            itype,
            encode(&name),
            search_args
        );
    }

    url
}

fn make_request(url: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let mut request_builder: RequestBuilder = client.get(&url);
    request_builder = request_builder.header("User-Agent", "MetacriticCLI");
    let response: Response = request_builder.send()?;
    let response_text = response.text()?;
    Ok(response_text)
}

fn scrap(
    document: &scraper::Html,
    number_of_results: usize,
    name: String,
) -> Vec<MetacriticResult> {
    let items_selector: scraper::Selector;
    let title_selector: scraper::Selector;
    let score_selector: scraper::Selector;
    let platform_selector: scraper::Selector;
    if name == String::from("coming-soon") {
        items_selector =
            scraper::Selector::parse("table.clamp-list>tbody>tr>td.clamp-summary-wrap").unwrap();
        title_selector = scraper::Selector::parse("a.title>h3").unwrap();
        platform_selector =
            scraper::Selector::parse("div.clamp-details>div.platform>span.data").unwrap();
        score_selector =
            scraper::Selector::parse("div.clamp-score-wrap>a.metascore_anchor>div.metascore_w")
                .unwrap();
    } else {
        items_selector = scraper::Selector::parse("ul.search_results.module>li.result").unwrap();
        title_selector = scraper::Selector::parse("h3.product_title>a").unwrap();
        score_selector = scraper::Selector::parse("div.main_stats>span.metascore_w").unwrap();
        platform_selector = scraper::Selector::parse("div.main_stats>p>span.platform").unwrap();
    }
    let items = document.select(&items_selector).map(|x| x.inner_html());

    let mut results: Vec<MetacriticResult> = Vec::new();

    items.zip(0..number_of_results).for_each(|(item, number)| {
        results.push(MetacriticResult::new(None, None, None));

        let current_item = scraper::Html::parse_document(&item);

        let titles = current_item.select(&title_selector).map(|x| x.inner_html());
        titles.zip(0..1).for_each(|(ite, _num)| {
            results[number].put_data(
                ite.trim()
                    .to_owned()
                    .replace("<span class=\"title_preifx\">", "")
                    .replace("</span>", ""),
                TSP::TITLE,
            );
        });

        let scores = current_item.select(&score_selector).map(|x| x.inner_html());
        scores
            .zip(0..)
            .for_each(|(ite, _num)| results[number].put_data(ite.trim().to_owned(), TSP::SCORE));

        let platforms = current_item
            .select(&platform_selector)
            .map(|x| x.inner_html());
        platforms
            .zip(0..)
            .for_each(|(ite, _num)| results[number].put_data(ite.trim().to_owned(), TSP::PLATFORM));
    });

    results
}

fn print_pretty(final_results: Vec<MetacriticResult>) {
    for result in final_results {
        if result.title == "" {
            break;
        }
        if result.score == "tbd" || result.score == "" {
            println!(
                "Title: {}\nScore: {}\nPlatform: {}\n\n",
                format!("{}", result.title).bold(),
                format!("{}", result.score),
                result.platform
            )
        } else if result.score.parse::<i32>().unwrap() > 74 {
            println!(
                "Title: {}\nScore: {}\nPlatform: {}\n\n",
                format!("{}", result.title).bold(),
                format!("{}", result.score).green(),
                result.platform
            )
        } else if result.score.parse::<i32>().unwrap() > 49
            && result.score.parse::<i32>().unwrap() < 75
        {
            println!(
                "Title: {}\nScore: {}\nPlatform: {}\n\n",
                format!("{}", result.title).bold(),
                format!("{}", result.score).yellow(),
                result.platform
            )
        } else {
            println!(
                "Title: {}\nScore: {}\nPlatform: {}\n\n",
                format!("{}", result.title).bold(),
                format!("{}", result.score).red(),
                result.platform
            )
        }
    }
}
