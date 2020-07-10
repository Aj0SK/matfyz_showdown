pub mod parse_html;
pub mod request;
pub mod secret;

use crate::request::helper;

use crate::parse_html::{parse_course, parse_main_page, parse_year};

use crate::secret::SECRET1;
use crate::secret::SECRET2;

//use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

use std::path::Path;

const MAX_COURSES: i32 = 100;
const DATA_PATH: &str = "./data";

fn main() {
    if Path::new(DATA_PATH).exists() {
        println!("Reuse old data.");
    } else {
        println!("Create new data.");
        prepare_data();
    }

    let data_paths = fs::read_dir(DATA_PATH).unwrap();

    let mut parsed_bodies: Vec<String> = Vec::new();
    let mut parsed_years: Vec<String> = Vec::new();
    let mut parsed_courses: Vec<String> = Vec::new();

    for path in data_paths {
        let path_str = path.unwrap().path().into_os_string().into_string().unwrap();
        let vec: Vec<&str> = path_str.split('&').collect();

        assert_eq!(vec.len(), 2);

        let year = vec[0];
        let course = vec[1];

        parsed_years.push(year.to_string());
        parsed_courses.push(course.to_string());

        let mut contents = String::new();
        File::open(path_str)
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
        parsed_bodies.push(contents.clone());
    }

    for i in parsed_bodies.iter() {
        parse_course(&i);
        //println!("{}", i);
        break;
    }

    println!("Len of parsed is {}", parsed_bodies.len());

    /*
    let mut mymap: HashMap<String, Vec<String>> = HashMap::new();
    for (key, value) in mymap.into_iter() {
        print!("{} ->", key);
        for v in value {
            print!(" {}", v);
        }
        println!();
    }
    for i in 0..year_links_abs.len() {
        for j in parse_year(&year_bodies[i]) {
            mymap.entry(j).or_default().push(year_links[i].clone());
        }
    }*/
}

fn prepare_data() {
    let cookie1 = format!("anketasessid_FMFI_prod={}", SECRET1);
    let cookie2 = format!("cosign-proxy-anketa.uniba.sk={}", SECRET2);
    let url = "https://anketa.uniba.sk/fmph/vysledky";

    let main_page: Vec<String> = vec![url.to_string()];
    let main_page_body: Vec<String> = helper(main_page, &cookie1, &cookie2);
    let year_links: Vec<String> = parse_main_page(&main_page_body[0]);

    let year_links_abs: Vec<String> = year_links
        .iter()
        .map(|x| format!("{}/{}/predmety/", url, x))
        .collect();
    let year_bodies: Vec<String> = helper(year_links_abs.clone(), &cookie1, &cookie2);

    fs::create_dir_all(DATA_PATH).unwrap();

    let mut parsed_urls: Vec<String> = Vec::new();
    let mut parsed_years: Vec<String> = Vec::new();
    let mut parsed_courses: Vec<String> = Vec::new();

    let mut counter: i32 = 0;

    'outer: for i in 0..year_links_abs.len() {
        for course_name in parse_year(&year_bodies[i]) {
            let curr_url = format!("{}/{}/predmet/{}", url, year_links[i], course_name);

            parsed_years.push(year_links[i].clone());
            parsed_courses.push(course_name.clone());
            parsed_urls.push(curr_url.clone());

            counter += 1;
            if counter == MAX_COURSES {
                break 'outer;
            }
        }
    }

    let requested_course_bodies: Vec<String> = helper(parsed_urls, &cookie1, &cookie2);

    for i in 0..requested_course_bodies.len() {
        let path = format!("{}/{}&{}", DATA_PATH, parsed_years[i], parsed_courses[i]);
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", path, why),
            Ok(file) => file,
        };
        file.write_all(&requested_course_bodies[i].as_bytes())
            .unwrap();
    }
}
