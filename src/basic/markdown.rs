use html2md::parse_html;
use regex::Regex;
use reqwest::blocking::get;
use std::env;
use std::fs;

fn check_url(url: &str) {
    if url.is_empty() {
        panic!("url is empty");
    }
    if !Regex::new(r"https?://\w+[a-z.0-9_][a-z.]{2,5}[-/\w0-9]*")
        .unwrap()
        .is_match(url)
    {
        panic!("url must be a valid url");
    }
}
fn check_output(output: &str) {
    if output.is_empty() {
        panic!("output is empty");
    }
    if !output.ends_with(".md") {
        panic!("output must end with .md");
    }
}

#[allow(dead_code)]
pub fn run() {
    let mut url: String = String::new();
    let mut output: String = String::new();
    let args: Vec<String> = env::args().collect();
    for i in 0..args.len() {
        let arg = args[i].as_str();
        match arg {
            "--url" => url = args[i + 1].to_string(),
            "--output" => output = args[i + 1].to_string(),
            _ => {}
        }
    }
    check_url(&url);
    check_output(&output);
    println!("Fetching url: {}", url);
    let body = get(url).unwrap().text().unwrap();
    println!("Converting html to markdown: {}", &body.replace("\n", ""));
    let md = parse_html(&body);
    fs::write(&output, md.as_bytes()).unwrap();
    println!("Converted markdown has been save in {}", output);
}
