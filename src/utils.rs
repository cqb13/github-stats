use reqwest::header::{HeaderMap, ACCEPT, USER_AGENT};
use serde_json::Value;
use std::path::{Path, PathBuf};

pub fn request(url: String) -> Result<Value, Box<dyn std::error::Error>> {
    let headers = construct_header();
    let client = reqwest::blocking::Client::new();
    let res = client.get(url).headers(headers).send()?;
    let body = res.text()?;
    let json = serde_json::from_str::<serde_json::Value>(&body)?;

    Ok(json)
}

fn construct_header() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "User".parse().unwrap());
    headers.insert(ACCEPT, "application/vnd.github.v3+json".parse().unwrap());
    return headers;
}

pub fn pretty_dates(date: &str) -> String {
    let date = date.split('T').collect::<Vec<&str>>();
    let date = date[0].split('-').collect::<Vec<&str>>();
    format!("{}-{}-{}", date[2], date[1], date[0])
}

pub fn validate_path(path: String) -> Result<PathBuf, String> {
    let real_path = Path::new(&path);

    if !real_path.exists() {
        return Err(format!("Failed to find path \"{}\"", path));
    }

    

    Ok(real_path.to_owned())
}
