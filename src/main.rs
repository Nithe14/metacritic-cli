//use clap;
mod args;

use args::Args;
use clap::Parser;
use colored::Colorize;
use urlencoding::encode;

fn main() {
    let args = Args::parse();
    //let mut hashMap: HashMap<String, String> = HashMap::new();
    let mut scores_vec: Vec<String> = vec![String::new(); 10];
    let mut titles_vec: Vec<String> = vec![String::new(); 10];
    let mut platforms_vec: Vec<String> = vec![String::new(); 10];
    let search_args;

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

    let response = reqwest::blocking::get(format!(
        "https://www.metacritic.com/search/{}/{}/results{}",
        args.itype,
        encode(&args.name),
        search_args
    ))
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&response);
    let items_selector = scraper::Selector::parse("ul.search_results.module>li.result").unwrap();
    let items = document.select(&items_selector).map(|x| x.inner_html());
    let score_selector = scraper::Selector::parse("div.main_stats>span.metascore_w").unwrap();
    let platform_selector = scraper::Selector::parse("div.main_stats>p>span.platform").unwrap();

    items.zip(1..11).for_each(|(item, number)| {
        //let score_selector = scraper:Selector::parse("div.main_stats>span.metascore_w").unwrap();
        let it = scraper::Html::parse_document(&item);
        let scores = it.select(&score_selector).map(|x| x.inner_html());
        scores
            .zip(1..11)
            .for_each(|(ite, _num)| scores_vec[number - 1] = ite.trim().to_owned());
        //println!("{}. {}", numer, ite.trim()));

        let platforms = it.select(&platform_selector).map(|x| x.inner_html());
        platforms
            .zip(0..10)
            .for_each(|(ite, _num)| platforms_vec[number - 1] = ite.trim().to_owned());

        let title_selector = scraper::Selector::parse("h3.product_title>a").unwrap();
        //let score_selector = scraper::Selector::parse("div.main_stats>span.metascore_w").unwrap();
        let titles = document.select(&title_selector).map(|x| x.inner_html());

        titles.zip(1..11).for_each(|(item, number)| {
            titles_vec[number - 1] = item
                .trim()
                .replace("<span class=\"title_prefix\">", "")
                .replace("</span>", "");
        });
    });

    if args.single {
        println!(
            "Title: {}\nScore: {}\nPlatform: {}\n\n",
            titles_vec[0], scores_vec[0], platforms_vec[0]
        )
    } else {
        for i in 0..args.number_of_results {
            //hashMap.insert(titles_vec[i].clone(), scores_vec[i].clone());
            if titles_vec[i] == "" {
                break;
            } else {
                if scores_vec[i].parse::<i32>().unwrap() > 74 {
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
    }
}
