use crate::utils::{pretty_dates, request, write_to_file};
use std::path::PathBuf;

pub fn user_command(user: String, output: Option<PathBuf>, display: bool) {
    let url = format!("https://api.github.com/users/{}", user);

    let json = request(url).expect("Failed to request data");

    if &json["message"] == "Not Found" {
        println!("User not found.");
        std::process::exit(0)
    }

    if display {
        // About
        let name = json["name"].as_str().unwrap_or("");
        let email = json["email"].as_str().unwrap_or("None");
        let bio = json["bio"].as_str().unwrap_or("None");
        let account_type = json["type"].as_str().unwrap_or("None");
        let url = json["html_url"].as_str().unwrap_or("None");
        let blog = json["blog"].as_str().unwrap_or("None");
        let company = json["company"].as_str().unwrap_or("None");
        let location = json["location"].as_str().unwrap_or("None");
        // User Stats
        let public_repos = json["public_repos"].as_i64().unwrap_or(0);
        let public_gists = json["public_gists"].as_i64().unwrap_or(0);
        let followers = json["followers"].as_i64().unwrap_or(0);
        let following = json["following"].as_i64().unwrap_or(0);
        // Important Dates
        let created_at = json["created_at"].as_str().unwrap_or("None");
        let updated_at = json["updated_at"].as_str().unwrap_or("None");

        println!(
            "ABOUT: {} {}",
            user,
            if name == "" {
                "".to_string()
            } else {
                format!("({})", name)
            }
        );
        if email != "" && email != "None" {
            println!("    {:<14}: {}", "Email", email);
        }
        println!("    {:<14}: {}", "Bio", bio);
        println!("    {:<14}: {}", "Account type", account_type);
        println!("    {:<14}: {}", "URL", url);
        if blog != "" && blog != "None" {
            println!("    {:<14}: {}", "Blog", blog);
        }
        if company != "null" && company != "None" {
            println!("    {:<14}: {}", "Company", company);
        }
        if location != "null" && location != "None" {
            println!("    {:<14}: {}", "Location", location);
        }
        println!();
        println!("USER STATS");
        println!("    {:<14}: {}", "Public repos", public_repos);
        println!("    {:<14}: {}", "Public gists", public_gists);
        println!("    {:<14}: {}", "Followers", followers);
        println!("    {:<14}: {}", "Following", following);
        println!();
        println!("IMPORTANT DATES");
        println!("    {:<14}: {}", "Created at", pretty_dates(created_at));
        println!("    {:<14}: {}", "Updated at", pretty_dates(updated_at));
        println!();
    } else {
        println!("{}", serde_json::to_string_pretty(&json).unwrap());
    }

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
