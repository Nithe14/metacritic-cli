mod args;

use args::Args;
use clap::Parser;
use colored::Colorize;
use reqwest::blocking::{RequestBuilder, Response};
use serde_json::{json, to_string, Map};
use urlencoding::encode;

enum TSP {
    TITLE,
    SCORE,
    PLATFORM,
}

#[derive(Debug)]
struct MetacriticResult {
    title: String,
    score: String,
    platform: String,
}
impl MetacriticResult {
    fn new(
        ititle: Option<String>,
        iscore: Option<String>,
        iplatform: Option<String>,
    ) -> MetacriticResult {
        MetacriticResult {
            title: ititle.unwrap_or(String::from("")),
            score: iscore.unwrap_or(String::from("")),
            platform: iplatform.unwrap_or(String::from("")),
        }
    }

    fn put_data(&mut self, input_data: String, dtype: TSP) {
        match dtype {
            TSP::TITLE => self.title = input_data,
            TSP::SCORE => self.score = input_data,
            TSP::PLATFORM => self.platform = input_data,
        }
    }
}

fn main() {
    let args = Args::parse();
    let mut_number_of_results: usize;
    let mut scores_vec: Vec<String> = vec![String::new(); 10];
    let mut titles_vec: Vec<String> = vec![String::new(); 10];
    let mut platforms_vec: Vec<String> = vec![String::new(); 10];
    let search_args;
    let mut json_vec = Vec::new();

    let mut mr: Vec<MetacriticResult> = vec![MetacriticResult::new(None, None, None)];

    match args.platform.as_str() {
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

    let response = make_request(args.name, args.itype, search_args).unwrap();

    let document = scraper::Html::parse_document(&response);
    let items_selector = scraper::Selector::parse("ul.search_results.module>li.result").unwrap();
    let items = document.select(&items_selector).map(|x| x.inner_html());
    let score_selector = scraper::Selector::parse("div.main_stats>span.metascore_w").unwrap();
    let platform_selector = scraper::Selector::parse("div.main_stats>p>span.platform").unwrap();

    items.zip(0..10).for_each(|(item, number)| {
        let it = scraper::Html::parse_document(&item);
        let scores = it.select(&score_selector).map(|x| x.inner_html());
        scores
            .zip(0..10)
            .for_each(|(ite, _num)| scores_vec[number] = ite.trim().to_owned());

        let platforms = it.select(&platform_selector).map(|x| x.inner_html());
        platforms
            .zip(0..10)
            .for_each(|(ite, _num)| platforms_vec[number] = ite.trim().to_owned());

        let title_selector = scraper::Selector::parse("h3.product_title>a").unwrap();
        let titles = document.select(&title_selector).map(|x| x.inner_html());

        titles.zip(0..10).for_each(|(item, number)| {
            titles_vec[number] = item
                .trim()
                .replace("<span class=\"title_prefix\">", "")
                .replace("</span>", "");
        });
    });

    if args.single {
        mut_number_of_results = 1;
    } else {
        mut_number_of_results = args.number_of_results;
    }
    for i in 0..mut_number_of_results {
        let mut hashmap = Map::new();

        if titles_vec[i] == "" {
            break;
        } else {
            if args.json {
                hashmap.insert("title".to_string(), json!(titles_vec[i]));
                hashmap.insert("score".to_string(), json!(scores_vec[i]));
                hashmap.insert("platform".to_string(), json!(platforms_vec[i]));
                json_vec.push(hashmap);
            } else if scores_vec[i] == "tbd" || scores_vec[i] == "" {
                println!(
                    "Title: {}\nScore: {}\nPlatform: {}\n\n",
                    format!("{}", titles_vec[i]).bold(),
                    format!("{}", scores_vec[i]),
                    platforms_vec[i]
                )
            } else if scores_vec[i].parse::<i32>().unwrap() > 74 {
                println!(
                    "Title: {}\nScore: {}\nPlatform: {}\n\n",
                    format!("{}", titles_vec[i]).bold(),
                    format!("{}", scores_vec[i]).green(),
                    platforms_vec[i]
                )
            } else if scores_vec[i].parse::<i32>().unwrap() > 49
                && scores_vec[i].parse::<i32>().unwrap() < 75
            {
                println!(
                    "Title: {}\nScore: {}\nPlatform: {}\n\n",
                    format!("{}", titles_vec[i]).bold(),
                    format!("{}", scores_vec[i]).yellow(),
                    platforms_vec[i]
                )
            } else {
                println!(
                    "Title: {}\nScore: {}\nPlatform: {}\n\n",
                    format!("{}", titles_vec[i]).bold(),
                    format!("{}", scores_vec[i]).red(),
                    platforms_vec[i]
                )
            }
        }
    }
    if args.json {
        println!("{}", to_string(&json_vec).unwrap());
    }
}

fn make_request(
    args_name: String,
    args_itype: String,
    search_args: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://www.metacritic.com/search/{}/{}/results{}",
        args_itype,
        encode(&args_name),
        search_args
    );
    let client = reqwest::blocking::Client::new();
    let mut request_builder: RequestBuilder = client.get(&url);
    request_builder = request_builder.header("User-Agent", "MetacriticCLI");
    let response: Response = request_builder.send()?;
    let response_text = response.text()?;
    Ok(response_text)
}

fn scrap(document: &scraper::Html, number_of_results: usize) {
    let items_selector = scraper::Selector::parse("ul.search_results.module>li.result").unwrap();
    let items = document.select(&items_selector).map(|x| x.inner_html());

    items.zip(0..number_of_results).for_each(|(item, number)| {
        let current_item = scraper::Html::parse_document(&item);

        let title_selector = scraper::Selector::parse("h3.product_title>a").unwrap();
        let titles = document.select(&title_selector).map(|x| x.inner_html());
        titles.zip(0..1).for_each(|(ite, _num)| {
            println!(
                "{}",
                ite.trim()
                    .to_owned()
                    .replace("<span class=\"title_preifx\">", "")
                    .replace("</span>", "")
            )
        });

        let score_selector = scraper::Selector::parse("div.main_stats>span.metascore_w").unwrap();
        let scores = current_item.select(&score_selector).map(|x| x.inner_html());
        scores
            .zip(0..)
            .for_each(|(ite, _num)| println!("{}", ite.trim().to_owned()));

        let platform_selector = scraper::Selector::parse("div.main_stats>p>span.platform").unwrap();
        let platforms = current_item
            .select(&platform_selector)
            .map(|x| x.inner_html());
        platforms
            .zip(0..)
            .for_each(|(ite, _num)| println!("{}", ite.trim().to_owned()));
    });
}
