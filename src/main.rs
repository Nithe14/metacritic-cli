//use clap;

fn main() {
    let mut scores_vec: Vec<String> = vec![String::new(); 10];
    let mut titles_vec: Vec<String> = vec![String::new(); 10];

    let response =
        reqwest::blocking::get("https://www.metacritic.com/search/all/bayonetta/results")
            .unwrap()
            .text()
            .unwrap();

    let document = scraper::Html::parse_document(&response);
    let items_selector = scraper::Selector::parse("ul.search_results.module>li.result").unwrap();
    let items = document.select(&items_selector).map(|x| x.inner_html());
    let score_selector = scraper::Selector::parse("div.main_stats>span.metascore_w").unwrap();

    items.zip(1..11).for_each(|(item, number)| {
        //let score_selector = scraper:Selector::parse("div.main_stats>span.metascore_w").unwrap();
        let it = scraper::Html::parse_document(&item);
        let scores = it.select(&score_selector).map(|x| x.inner_html());
        scores
            .zip(1..11)
            .for_each(|(ite, num)| scores_vec[number - 1] = ite.trim().to_owned());
        //println!("{}. {}", numer, ite.trim()));
    });

    let title_selector = scraper::Selector::parse("h3.product_title>a").unwrap();
    //let score_selector = scraper::Selector::parse("div.main_stats>span.metascore_w").unwrap();
    let titles = document.select(&title_selector).map(|x| x.inner_html());

    titles.zip(1..11).for_each(|(item, number)| {
        titles_vec[number - 1] = item
            .trim()
            .replace("<span class=\"title_prefix\">", "")
            .replace("</span>", "");
    });

    for i in 1..11 {
        println!("Title: {}", titles_vec[i - 1]);
        println!("Score: {}", scores_vec[i - 1]);
    }
}
