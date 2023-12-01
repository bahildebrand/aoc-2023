use std::env;
use std::fs::File;
use std::io::Write;

use reqwest::blocking::Client;

pub fn fetch_input(day: usize) {
    let cookie = env::var("AOC_COOKIE").expect("AOC_COOKIE not set");
    let url = format!("https://adventofcode.com/2023/day/{}/input", day);

    let client = Client::new();
    let body = client
        .get(&url)
        .header("Cookie", cookie)
        .send()
        .expect("Failed to send request")
        .text()
        .expect("Failed to get response text");

    let file_name = format!("input/day-{}.txt", day);
    let mut file = File::create(&file_name).expect("Failed to create file");

    file.write_all(body.as_bytes())
        .expect("Failed to write to file");
    println!("Wrote input to {}", file_name);
}

pub fn get_input(day: usize) -> String {
    let file_name = format!("input/day-{}.txt", day);
    std::fs::read_to_string(&file_name).expect("Failed to read input file")
}
