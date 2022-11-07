use clap;

fn main() {
    let response =
        reqwest::blocking::get("https://www.metacritic.com/search/all/bayonetta/results")
            .unwrap()
            .text()
            .unwrap();

    let document = scraper::Html::parse_document(&response);
    let title_selector = scraper::Selector::parse("h3.product_title>a").unwrap();
    let score_selector = scraper::Selector::parse("div.main_stats>span.metascore_w").unwrap();
    let titles = document.select(&title_selector).map(|x| x.inner_html());

    titles.zip(1..11).for_each(|(item, number)| {
        println!(
            "{}. {}",
            number,
            item.trim()
                .replace("<span class=\"title_prefix\">", "")
                .replace("</span>", "")
        )
    });
}
