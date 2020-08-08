mod types;

use reqwest;
use std::fs::File;
use std::io::prelude::*;
use url;

fn main() {
    let mut file = File::open("words.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    let words: Vec<&str> = contents.split("\n").collect();

    for word in words.iter() {
        let word_url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);
        let parsed_url = url::Url::parse(&word_url).unwrap();
        let body = reqwest::blocking::get(parsed_url).unwrap();
        println!("{}", body.text().unwrap());
    }
    
    

   
}