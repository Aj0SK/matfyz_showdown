extern crate futures;
extern crate reqwest;

use futures::{stream, StreamExt};
use http::{header::COOKIE, HeaderValue};

#[tokio::main]
pub async fn helper(
    urls: Vec<String>,
    cookie1: &str,
    cookie2: &str,
) -> std::vec::Vec<std::string::String> {
    use reqwest::header;
    use reqwest::Client;

    let client = Client::new();

    let bodies = stream::iter(urls)
        .map(|url| {
            let client = &client;
            let mut headers = header::HeaderMap::new();
            headers.insert(COOKIE, HeaderValue::from_str(&cookie1).unwrap());
            headers.insert(COOKIE, HeaderValue::from_str(&cookie2).unwrap());

            async move {
                let resp = client.get(&url).headers(headers).send().await?;
                resp.text().await
            }
        })
        .buffer_unordered(16);

    let mut results: Vec<String> = vec![];

    let _work = bodies
        .for_each(|b| {
            match b {
                Ok(b) => results.push(b),
                Err(e) => eprintln!("Error: {}", e),
            }
            async { () }
        })
        .await;

    return results;
}
