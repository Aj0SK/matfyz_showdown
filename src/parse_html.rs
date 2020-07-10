extern crate scraper;

use scraper::{Html, Selector};

pub fn parse_year(body: &str) -> Vec<String> {
    let fragment = Html::parse_document(&body);
    let stories1 = Selector::parse(r#"#content>ul>li>a"#).unwrap();

    let mut results: Vec<String> = vec![];

    for story in fragment.select(&stories1) {
        let x = story.value().attr("href").unwrap();
        //println!("{}", x.to_string());
        let y = x.to_string().rfind("/predmet/").unwrap_or(0);
        //println!("{}", &x.to_string()[y+9..]);
        results.push(x.to_string()[y + 9..].to_string());
    }

    results
}

pub fn parse_main_page(body: &str) -> Vec<String> {
    let fragment = Html::parse_document(&body);
    let stories1 = Selector::parse(r#"a[class="menu-level-0 leaf"]"#).unwrap();
    let mut results: Vec<String> = vec![];

    for story in fragment.select(&stories1) {
        let x = story.value().attr("href").unwrap();
        let y = x.to_string().rfind("/vysledky/").unwrap_or(0);
        results.push(x.to_string()[y + 10..].to_string());
    }

    results
}

pub fn parse_course(body: &str) {
    let fragment = Html::parse_document(&body);
    let stories1 = Selector::parse(r#"h2[class="fragment"]"#).unwrap();
    let stories2 = Selector::parse(r#"li[data-cnt]"#).unwrap();
    let stories3 = Selector::parse(r#"li[data-avg]"#).unwrap();

    for story in fragment.select(&stories1) {
        let story_txt = story.text().collect::<Vec<_>>();
        println!("{:?}", story_txt[1].trim());
    }

    for story in fragment.select(&stories2) {
        let story_txt = story.text().collect::<Vec<_>>();
        println!("{:?}", story_txt[0].trim());
    }

    for story in fragment.select(&stories3) {
        let story_txt = story.text().collect::<Vec<_>>();
        println!("{:?}", story_txt[0].trim());
    }
}
