extern crate reqwest;
extern crate scraper;
extern crate futures;

pub mod secret;

use crate::secret::SECRET1;
use crate::secret::SECRET2;

use scraper::{Html, Selector};
use http::{HeaderValue, header::{COOKIE}};
use futures::{stream, StreamExt};

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

const MAX_COURSES: i32 = 10;

fn main() {
    let cookie1 = format!("anketasessid_FMFI_prod={}", SECRET1);
    let cookie2 = format!("cosign-proxy-anketa.uniba.sk={}", SECRET2);
    let url = "https://anketa.uniba.sk/fmph/vysledky";
    //let base = "https://anketa.uniba.sk";

    //get_course(&url, &cookie1, &cookie2);

    let main_page : Vec<String> =  vec![url.to_string()];
    let main_page_body: Vec<String> = helper(main_page, &cookie1, &cookie2);
    let year_links: Vec<String> = parse_main_page(&main_page_body[0]);

    let year_links_abs: Vec<String> = year_links.clone().iter().map(|x| format!("{}/{}/predmety/", url, x)).collect();
    let year_bodies: Vec<String> = helper(year_links_abs.clone(), &cookie1, &cookie2);

    let mut mymap : HashMap<String, Vec<String>> = HashMap::new();

    for i in 0..year_links_abs.len() {
        let x = parse_year(&year_bodies[i]);
        for j in x {
            mymap.entry(j).or_default().push(year_links[i].clone());
        }
    }

    fs::create_dir_all("./data").unwrap();

    let mut parsed_urls: Vec<String> = Vec::new();
    let mut parsed_years: Vec<String> = Vec::new();
    let mut parsed_courses: Vec<String> = Vec::new();

    let mut counter: i32 = 0;
    'outer: for (key, value) in mymap.into_iter() {
        print!("{} ->", key);
        for v in value {
            print!(" {}", v);

            let curr_url = format!("{}/{}/predmet/{}", url, v, key);

            parsed_years.push(v.clone());
            parsed_courses.push(key.clone());
            parsed_urls.push(curr_url.clone());

            counter += 1;
            if counter == MAX_COURSES {
                break 'outer;
            }
        }
        println!();
    }

    let requested_course_bodies: Vec<String> = helper(parsed_urls, &cookie1, &cookie2);

    for i in 0..requested_course_bodies.len() {
        let path = format!("./data/{}&{}", parsed_years[i], parsed_courses[i]);
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", path, why),
            Ok(file) => file,
        };
        file.write_all(&requested_course_bodies[i].as_bytes()).unwrap();
    }

    println!("\nCounter is {}", counter);
}

fn parse_year(body: &str) -> Vec<String> {

    let fragment = Html::parse_document(&body);
    let stories1 = Selector::parse(r#"#content>ul>li>a"#).unwrap();

    let mut results: Vec<String> = vec![];

    for story in fragment.select(&stories1) {
        let x = story.value().attr("href").unwrap();
        //println!("{}", x.to_string());
        let y = x.to_string().rfind("/predmet/").unwrap_or(0);
        //println!("{}", &x.to_string()[y+9..]);
        results.push(x.to_string()[y+9..].to_string());
    }

    return results;
}

fn parse_main_page(body: &str) -> Vec<String> {

    let fragment = Html::parse_document(&body);
    let stories1 = Selector::parse(r#"a[class="menu-level-0 leaf"]"#).unwrap();
    let mut results: Vec<String> = vec![];

    for story in fragment.select(&stories1) {
        let x = story.value().attr("href").unwrap();
        let y = x.to_string().rfind("/vysledky/").unwrap_or(0);
        results.push(x.to_string()[y+10..].to_string());
    }

    return results;
}

#[tokio::main]
async fn helper(urls: Vec<String>, cookie1: &str, cookie2: &str) -> std::vec::Vec<std::string::String> {

    use reqwest::Client;
    use reqwest::header;
    
    let client = Client::new();

    let bodies = stream::iter(urls)
        .map(|url| {
            let client = &client;
            let mut headers = header::HeaderMap::new();
            headers.insert(COOKIE, HeaderValue::from_str(&cookie1).unwrap());
            headers.insert(COOKIE, HeaderValue::from_str(&cookie2).unwrap());
                
            async move {
                let resp = client.get(&url)
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

/*fn parse_course(body: &str) {

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

}*/

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
