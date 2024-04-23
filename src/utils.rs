use reqwest::header::{HeaderMap, ACCEPT, USER_AGENT};
use serde_json::Value;
use std::fs::{create_dir_all, remove_file, File};
use std::io::{BufWriter, Write};
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
    headers.insert(USER_AGENT, "GSTATS".parse().unwrap());
    headers.insert(ACCEPT, "application/vnd.github.v3+json".parse().unwrap());
    return headers;
}

pub fn pretty_dates(date: &str) -> String {
    let date = date.split('T').collect::<Vec<&str>>();
    let date = date[0].split('-').collect::<Vec<&str>>();
    format!("{}-{}-{}", date[2], date[1], date[0])
}

pub fn validate_and_convert_path(path: String) -> Result<PathBuf, String> {
    let real_path = Path::new(&path);

    if real_path.extension().is_some() && real_path.extension().unwrap() != "json" {
        return Err("The file must be a json file.".to_owned());
    }

    if real_path.is_file() {
        println!("A file was found at the path: \"{}\"", path);
        println!("Would you like to clear the file and continue? [y/N]");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() != "y" {
            return Err("A file already exists at the path.".to_owned());
        } else {
            remove_file(&real_path).expect("Failed to delete existing directory");
        }
    }

    Ok(real_path.to_owned())
}

pub fn write_json_to_file(json: Value, mut path: PathBuf) -> Result<(), String> {
    if path.extension().is_none() {
        path.push("gstats-output.json");
    }

    println!("Saving the json to: {:?}", path);
    create_dir_all(path.parent().unwrap()).unwrap();
    let file = match File::create(&path) {
        Ok(file) => file,
        Err(_) => {
            return Err("Failed to create the file.".to_owned());
        }
    };

    let mut writer = BufWriter::new(file);
    let json_string = serde_json::to_string_pretty(&json).unwrap();
    writer.write_all(json_string.as_bytes()).unwrap();

    Ok(())
}
