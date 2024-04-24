use crate::utils::{bytes_to_best_size, pretty_dates, request, write_json_to_file};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::{path::PathBuf, vec};

pub struct DownloadData {
    pub name: String,
    pub tag: String,
    pub published_at: String,
    pub created_at: String,
    pub html_url: String,
    pub body: String,
    pub assets: Vec<Asset>,
    pub downloads: i64,
}

impl DownloadData {
    pub fn new(
        name: String,
        tag: String,
        published_at: String,
        created_at: String,
        html_url: String,
        body: String,
    ) -> DownloadData {
        DownloadData {
            name,
            tag,
            published_at,
            created_at,
            html_url,
            body,
            assets: Vec::new(),
            downloads: 0,
        }
    }

    pub fn display(&self) {
        println!("{} - {:>10}", self.name.to_ascii_uppercase(), self.tag);
        if self.body != "" {
            println!("    {}", self.body);
        }
        println!("    {:<14}: {}", "HTML URL", self.html_url);
        println!("    {:<14}: {}", "Downloads", self.downloads);
        println!("    {:<14}: {}", "Created at", self.created_at);
        println!("    {:<14}: {}", "Published at", self.published_at);
        println!("    {:<14}: {}", "ASSETS", self.assets.len());
        for asset in &self.assets {
            asset.display();
        }
        println!();
    }
}

pub struct Asset {
    pub download_url: String,
    pub created_at: String,
    pub downloads: i64,
    pub name: String,
    pub size: i64,
    pub updated_at: String,
}

impl Asset {
    pub fn new(
        download_url: String,
        created_at: String,
        downloads: i64,
        name: String,
        size: i64,
        updated_at: String,
    ) -> Asset {
        Asset {
            download_url,
            created_at,
            downloads,
            name,
            size,
            updated_at,
        }
    }

    pub fn display(&self) {
        println!(
            "    {:<14}: {}",
            self.name.to_uppercase(),
            bytes_to_best_size(self.size)
        );
        println!("        {:<14}: {}", "Download URL", self.download_url);
        println!("        {:<14}: {}", "Downloads", self.downloads);
        println!("        {:<14}: {}", "Created at", self.created_at);
        println!("        {:<14}: {}", "Updated at", self.updated_at);
        println!()
    }
}

pub fn downloads_command(
    owner: String,
    repo: String,
    individual: bool,
    link: bool,
    output: Option<PathBuf>,
    all: bool,
    display: bool,
) {
    let url = format!("https://api.github.com/repos/{}/{}/releases", owner, repo);

    let json = request(url).expect("Failed to request data");

    if &json["message"] == "Not Found" {
        println!("Repository not found.");
        std::process::exit(0)
    }

    if json.as_array().unwrap().is_empty() {
        println!("No releases found.");
        std::process::exit(0)
    }

    if all && !display {
        println!("{:#?}", json);
    } else if all && display {
        let simple_data = simplify_json_release_data(&json);
        let mut download_count = 0;
        for release in simple_data {
            release.display();
            download_count += release.downloads;
        }
        println!("Total Downloads: {}", download_count);
    } else if !all && !display {
        // if individual, get latest download url, and sum all item count
        // {download_url: Vec<(String name, String link for each asset of latest)>, html_url: String, download_count: i32}
    }

    match output {
        Some(path) => {
            let result = write_json_to_file(json, path);
            match result {
                Ok(_) => {}
                Err(err) => println!("{}", err),
            }
        }
        None => {}
    }
}

fn simplify_json_release_data(json: &Value) -> Vec<DownloadData> {
    let mut download_data: Vec<DownloadData> = Vec::new();

    for release in json.as_array().unwrap() {
        let name = &release["name"].as_str().unwrap_or("None").to_string();
        let tag = &release["tag_name"].as_str().unwrap_or("None").to_string();
        let published_at = &release["published_at"]
            .as_str()
            .unwrap_or("None")
            .to_string();
        let created_at = &release["created_at"].as_str().unwrap_or("None").to_string();
        let html_url = &release["html_url"].as_str().unwrap_or("None").to_string();
        let body = &release["body"].as_str().unwrap_or("None").to_string();

        let mut download = DownloadData::new(
            name.to_string(),
            tag.to_string(),
            pretty_dates(&published_at),
            pretty_dates(&created_at),
            html_url.to_string(),
            body.to_string(),
        );

        for asset in release["assets"].as_array().unwrap() {
            let download_url = &asset["browser_download_url"]
                .as_str()
                .unwrap_or("None")
                .to_string();
            let created_at = &asset["created_at"].as_str().unwrap_or("None").to_string();
            let downloads = &asset["download_count"].as_i64().unwrap_or(0);
            let name = &asset["name"].as_str().unwrap_or("None").to_string();
            let size = &asset["size"].as_i64().unwrap_or(0);
            let updated_at = &asset["updated_at"].as_str().unwrap_or("None").to_string();

            let asset = Asset::new(
                download_url.to_string(),
                pretty_dates(&created_at),
                downloads.to_owned(),
                name.to_string(),
                size.to_owned(),
                pretty_dates(&updated_at),
            );

            download.downloads += downloads;
            download.assets.push(asset);
        }

        download_data.push(download);
    }

    download_data
}
