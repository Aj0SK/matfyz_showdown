pub mod parse_html;
pub mod request;
pub mod secret;

use crate::request::helper;

use crate::parse_html::parse_main_page;
use crate::parse_html::parse_year;

use crate::secret::SECRET1;
use crate::secret::SECRET2;

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

    let main_page: Vec<String> = vec![url.to_string()];
    let main_page_body: Vec<String> = helper(main_page, &cookie1, &cookie2);
    let year_links: Vec<String> = parse_main_page(&main_page_body[0]);

    let year_links_abs: Vec<String> = year_links
        .clone()
        .iter()
        .map(|x| format!("{}/{}/predmety/", url, x))
        .collect();
    let year_bodies: Vec<String> = helper(year_links_abs.clone(), &cookie1, &cookie2);

    let mut mymap: HashMap<String, Vec<String>> = HashMap::new();

    for i in 0..year_links_abs.len() {
        for j in parse_year(&year_bodies[i]) {
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
        file.write_all(&requested_course_bodies[i].as_bytes())
            .unwrap();
    }

    println!("\nCounter is {}", counter);
}
