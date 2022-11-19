//use clap;
mod args;

use args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    //let mut hashMap: HashMap<String, String> = HashMap::new();
    let mut scores_vec: Vec<String> = vec![String::new(); 10];
    let mut titles_vec: Vec<String> = vec![String::new(); 10];
    let mut platforms_vec: Vec<String> = vec![String::new(); 10];

    let response = reqwest::blocking::get(format!(
        "https://www.metacritic.com/search/all/{}/results",
        args.name
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
            println!(
                "Title: {}\nScore: {}\nPlatform: {}\n\n",
                titles_vec[i], scores_vec[i], platforms_vec[i]
            )
        }
    }
}
