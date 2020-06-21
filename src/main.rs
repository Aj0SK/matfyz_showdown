extern crate reqwest;
extern crate scraper;

// importation syntax 
use scraper::{Html, Selector};
use http::{HeaderValue, header::{COOKIE}};

fn main() {
    let secret1 = "secret1";
    let secret2 = "secret2";
    let cookie1 = format!("anketasessid_FMFI_prod={}", secret1);
    let cookie2 = format!("cosign-proxy-anketa.uniba.sk={}", secret2);
    let url = "https://anketa.uniba.sk/fmph/vysledky/2018-2019-leto/predmet/1-INF-156--10";
    
    get_course(&url, &cookie1, &cookie2);
}

fn get_course(url: &str, cookie1: &str, cookie2: &str) {
    use reqwest::blocking::Client;
    use reqwest::header;
    
    let mut headers = header::HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&cookie1).unwrap());
    headers.insert(COOKIE, HeaderValue::from_str(&cookie2).unwrap());
    
    let client = Client::new();
    let res = client.get(url)
    .headers(headers)
    .send()
    .unwrap();

    let body = res.text().unwrap();

    println!("{0}", body);
    
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
