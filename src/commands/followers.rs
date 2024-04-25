use crate::utils::{request, write_to_file};
use std::path::PathBuf;

pub fn followers_command(user: String, total: bool, output: Option<PathBuf>, display: bool) {
    let url = format!("https://api.github.com/users/{}/followers", user);

    let json = request(url).expect("Failed to request data");

    if &json["message"] == "Not Found" {
        println!("User not found.");
        std::process::exit(0)
    }

    if display {
        for follower in json.as_array().unwrap() {
            let username = follower["login"].as_str().unwrap_or("None");
            let html_url = follower["html_url"].as_str().unwrap_or("None");
            println!("{:<20} - {}", username, html_url);
            println!();
        }
        if total {
            println!("Followers: {}", json.as_array().unwrap().len())
        }
    } else {
        let mut total_json = serde_json::Map::new();
        total_json.insert("followers".to_string(), json.clone());
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
        total_json.insert("followers".to_string(), json);

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
