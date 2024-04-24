use reqwest::header::{HeaderMap, ACCEPT, USER_AGENT};
use serde_json::Value;
use std::env;
use std::fs;
use std::fs::{create_dir_all, remove_file, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum OS {
    Windows,
    Mac,
}

impl OS {
    fn get_name(&self) -> &str {
        match self {
            OS::Windows => "Windows",
            OS::Mac => "Mac",
        }
    }
}

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

pub fn write_to_file(json_string: String, mut path: PathBuf) -> Result<(), String> {
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
    let json_string = json_string;
    writer.write_all(json_string.as_bytes()).unwrap();

    Ok(())
}

pub fn bytes_to_best_size(bytes: i64) -> String {
    let mut size = bytes as f64;
    let mut unit = "B";

    if size > 1024.0 {
        size /= 1024.0;
        unit = "KB";
    }

    if size > 1024.0 {
        size /= 1024.0;
        unit = "MB";
    }

    if size > 1024.0 {
        size /= 1024.0;
        unit = "GB";
    }

    if size > 1024.0 {
        size /= 1024.0;
        unit = "TB";
    }

    format!("{:.2} {}", size, unit)
}

pub fn install(os: &OS) {
    println!("starting install on {}", os.get_name());

    let home_dir = get_user_home_dir(os);
    let bin_path = match os {
        OS::Windows => "AppData/Roaming/gstats",
        OS::Mac => ".local/bin",
    };

    let local_bin_path = format!("{}/{}", home_dir, bin_path);

    // Create directory if it doesn't exist
    if !Path::new(&local_bin_path).exists() {
        println!("Creating {} directory", local_bin_path);
        fs::create_dir_all(&local_bin_path).unwrap();
    }

    let binary_name = match os {
        OS::Windows => "gstats.exe",
        OS::Mac => "gstats",
    };

    let new_binary_path = format!("{}/{}", local_bin_path, binary_name);

    if Path::new(&new_binary_path).exists() {
        println!("Replacing binary in {}", &local_bin_path);
        fs::remove_file(&new_binary_path).unwrap();
    }

    println!("Moving binary to {}", local_bin_path);
    fs::copy(
        format!("{}/{}", get_current_directory_path(), binary_name),
        &new_binary_path,
    )
    .unwrap();

    match os {
        OS::Windows => {
            if let Err(e) = add_registry_path(&local_bin_path) {
                eprintln!("Failed to modify system PATH: {}", e);
                eprintln!("This action may require administrator permissions.");
                return;
            }
        }
        OS::Mac => {
            let zprofile_path = format!("{}/.zprofile", home_dir);
            if let Ok(zprofile_content) = fs::read_to_string(&zprofile_path) {
                if !zprofile_content.contains("export PATH=\"$PATH:$HOME/.local/bin\"") {
                    println!("Adding .local/bin to path in .zprofile");
                    let mut zprofile_file = File::create(&zprofile_path).unwrap();
                    writeln!(zprofile_file, "export PATH=\"$PATH:$HOME/.local/bin\"").unwrap();
                }
            }
        }
    }

    println!("install complete");
}

fn add_registry_path(new_path: &str) -> std::io::Result<()> {
    use std::process::Command;

    // Escape percent signs by doubling them
    let escaped_path = new_path.replace("%", "%%");

    // Prepare the command to modify the registry
    let status = Command::new("reg")
        .args(&[
            "ADD",
            "HKLM\\SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment",
            "/v",
            "Path",
            "/t",
            "REG_EXPAND_SZ",
            "/d",
            &escaped_path,
            "/f",
        ])
        .status()?;

    if !status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to modify registry",
        ));
    }

    Ok(())
}

fn get_current_directory_path() -> String {
    let current_dir_path = match env::current_dir() {
        Ok(path) => path,
        Err(_) => panic!("Could not get current directory path"),
    };

    current_dir_path.to_str().unwrap().to_string()
}

fn get_user_home_dir(os: &OS) -> String {
    let user_dir = match os {
        OS::Windows => "USERPROFILE",
        OS::Mac => "HOME",
    };
    std::env::var(format!("{}", user_dir)).unwrap()
}
