use crate::utils::{pretty_dates, request, write_to_file};
use serde_json::Value;
use std::path::PathBuf;

pub fn repo_command(owner: String, repo: String, output: Option<PathBuf>, display: bool) {
    let url = format!("https://api.github.com/repos/{}/{}", owner, repo);

    let json = request(url).expect("Failed to request data");

    if &json["message"] == "Not Found" {
        println!("Repository not found.");
        std::process::exit(0)
    }

    if !display {
        println!("{:#?}", json);
    } else {
        simplify_and_display_json(&json)
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

fn simplify_and_display_json(json: &Value) {
    // General Info
    let name = json["name"].as_str().unwrap_or("None");
    let description = json["description"].as_str().unwrap_or("None");
    // Repo Status
    let forking = json["allow_forking"].as_bool().unwrap();
    let archived = json["archived"].as_bool().unwrap();
    let fork = json["fork"].as_bool().unwrap();
    let disabled = json["disabled"].as_bool().unwrap();
    let private = json["private"].as_bool().unwrap();
    // Activity Metrics
    let stars = json["stargazers_count"].as_i64().unwrap_or(0);
    let forks = json["forks_count"].as_i64().unwrap_or(0);
    let watches = json["watchers_count"].as_i64().unwrap_or(0);
    let open_issues = json["open_issues_count"].as_i64().unwrap_or(0);
    // Repository URLs
    let html_url = json["html_url"].as_str().unwrap_or("None");
    let clone_url = json["clone_url"].as_str().unwrap_or("None");
    let homepage = json["homepage"].as_str().unwrap_or("None");
    // Development Details
    let language = json["language"].as_str().unwrap_or("None");
    let default_branch = json["default_branch"].as_str().unwrap_or("None");
    let license = json["license"]["name"].as_str().unwrap_or("None");
    // Important Dates
    let created_at = json["created_at"].as_str().unwrap_or("None");
    let updated_at = json["updated_at"].as_str().unwrap_or("None");
    let pushed_at = json["pushed_at"].as_str().unwrap_or("None");

    println!("GENERAL INFO");
    println!("    {:<14}: {}", "Name", name);
    println!("    {:<14}: {}", "Description", description);
    println!();
    println!("REPO STATUS");
    println!("    {:<14}: {}", "Allows forks", forking);
    println!("    {:<14}: {}", "Archived", archived);
    println!("    {:<14}: {}", "Fork", fork);
    println!("    {:<14}: {}", "Disabled", disabled);
    println!("    {:<14}: {}", "Private", private);
    println!();
    println!("ACTIVITY METRICS");
    println!("    {:<14}: {}", "Stars", stars);
    println!("    {:<14}: {}", "Forks", forks);
    println!("    {:<14}: {}", "Watches", watches);
    println!("    {:<14}: {}", "Open issues", open_issues);
    println!();
    println!("REPOSITORY URLS");
    println!("    {:<14}: {}", "HTML URL", html_url);
    println!("    {:<14}: {}", "Clone URL", clone_url);
    if homepage != "" && homepage != "None" {
        println!("    {:<14}: {}", "Homepage", homepage);
    }
    println!();
    println!("DEVELOPMENT DETAILS");
    println!("    {:<14}: {}", "Language", language);
    println!("    {:<14}: {}", "Default branch", default_branch);
    if license != "None" {
        println!("    {:<14}: {}", "License", license);
    }
    println!();
    println!("IMPORTANT DATES");
    println!("    {:<14}: {}", "Created at", pretty_dates(created_at));
    println!("    {:<14}: {}", "Updated at", pretty_dates(updated_at));
    println!("    {:<14}: {}", "Pushed at", pretty_dates(pushed_at));
    println!();
}
