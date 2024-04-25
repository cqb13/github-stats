use crate::utils::{request, write_to_file};
use std::path::PathBuf;

pub enum RelationType {
    Follower,
    Following,
}

impl RelationType {
    pub fn to_text(&self) -> String {
        match self {
            RelationType::Follower => "Followers".to_string(),
            RelationType::Following => "Following".to_string(),
        }
    }
}

pub fn relations_command(
    user: String,
    total: bool,
    output: Option<PathBuf>,
    display: bool,
    relation_type: RelationType,
) {
    let url = format!(
        "https://api.github.com/users/{}/{}",
        user,
        relation_type.to_text().to_ascii_lowercase()
    );

    let json = request(url).expect("Failed to request data");

    if &json["message"] == "Not Found" {
        println!("User not found.");
        std::process::exit(0)
    }

    if display {
        for relation in json.as_array().unwrap() {
            let username = relation["login"].as_str().unwrap_or("None");
            let html_url = relation["html_url"].as_str().unwrap_or("None");
            println!("{:<20} - {}", username, html_url);
            println!();
        }
        if total {
            println!(
                "{}: {}",
                relation_type.to_text(),
                json.as_array().unwrap().len()
            )
        }
    } else {
        let mut total_json = serde_json::Map::new();
        total_json.insert(relation_type.to_text().to_ascii_lowercase(), json.clone());
        if total {
            total_json.insert(
                "total".to_string(),
                serde_json::Value::Number(serde_json::Number::from(json.as_array().unwrap().len())),
            );
        }

        println!("{}", serde_json::to_string_pretty(&total_json).unwrap())
    }

    if total {
        let mut total_json = serde_json::Map::new();
        total_json.insert(
            "total".to_string(),
            serde_json::Value::Number(serde_json::Number::from(json.as_array().unwrap().len())),
        );
        total_json.insert(relation_type.to_text().to_ascii_lowercase(), json);

        match output {
            Some(path) => {
                let result =
                    write_to_file(serde_json::to_string_pretty(&total_json).unwrap(), path);
                match result {
                    Ok(_) => {}
                    Err(err) => println!("{}", err),
                }
            }
            None => {}
        }
    } else {
        match output {
            Some(path) => {
                let result = write_to_file(serde_json::to_string_pretty(&json).unwrap(), path);
                match result {
                    Ok(_) => {}
                    Err(err) => println!("{}", err),
                }
            }
            None => {}
        }
    }
}
