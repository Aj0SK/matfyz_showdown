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
    let url = "https://anketa.uniba.sk/fmph/vysledky";
    
    hn_headlines(&url, &cookie1, &cookie2);
}

fn hn_headlines(url: &str, cookie1: &str, cookie2: &str) {
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
    // parses based on a CSS selector
    let stories = Selector::parse(".storylink").unwrap();

    // iterate over elements matching our selector
    for story in fragment.select(&stories) {
        // grab the headline text and place into a vector
        let story_txt = story.text().collect::<Vec<_>>();
        println!("{:?}", story_txt);
    }
}
