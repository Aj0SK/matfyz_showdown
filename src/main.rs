extern crate reqwest;
extern crate scraper;
extern crate futures;

pub mod secret;

use crate::secret::SECRET1;
use crate::secret::SECRET2;

use scraper::{Html, Selector};
use http::{HeaderValue, header::{COOKIE}};
use futures::{stream, StreamExt};

fn main() {
    let cookie1 = format!("anketasessid_FMFI_prod={}", SECRET1);
    let cookie2 = format!("cosign-proxy-anketa.uniba.sk={}", SECRET2);
    let url = "https://anketa.uniba.sk/fmph/vysledky/2018-2019-leto/predmet/1-INF-156--10";
    
    //get_course(&url, &cookie1, &cookie2);
    
    let mut urls = vec![];
    let n = 2;
    for i in 0..n {
        urls.push(url);
    }
    let bodies: Vec<String> = helper(&urls, &cookie1, &cookie2);
    for i in bodies {
        parse_course(&i);
    }
}

#[tokio::main]
async fn helper(urls: &[& str], cookie1: &str, cookie2: &str) -> std::vec::Vec<std::string::String> {

    use reqwest::Client;
    use reqwest::header;
    
    let client = Client::new();

    let bodies = stream::iter(urls)
        .map(|&url| {
            let client = &client;
            let mut headers = header::HeaderMap::new();
            headers.insert(COOKIE, HeaderValue::from_str(&cookie1).unwrap());
            headers.insert(COOKIE, HeaderValue::from_str(&cookie2).unwrap());
                
            async move {
                let resp = client.get(url)
                .headers(headers)
                .send()
                .await?;
                resp.text().await
            }
        }).buffer_unordered(16);

    let mut results: Vec<String> = vec![];
   
    let _work = bodies
    .for_each(|b| {
        match b {
            Ok(b) => {
                results.push(b)
            },
            Err(e) => eprintln!("Error: {}", e),
        }
        async { () }
    })
    .await;
    

    return results;
}

fn parse_course(body: &str) {

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

/*fn get_course(url: &str, cookie1: &str, cookie2: &str) {
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

    parse_course(&body);
}*/
