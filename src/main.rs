pub mod cli;
pub mod commands;
pub mod utils;

use crate::cli::{Arg, Cli, Command};
use crate::commands::all::all_command;
use crate::commands::releases::releases_command;
use crate::commands::user::user_command;
use crate::utils::{install, validate_and_convert_path, OS};

fn main() {
    let cli = Cli::new().with_default_command("help").with_commands(vec![
        Command::new("version", "Displays the current version of github-stats.").with_short('v'),
        Command::new("install", "Installs the files and directories."),
        Command::new("all", "Gives all stats found on a repository as json.")
            .with_arg(
                Arg::new()
                    .with_name("user")
                    .with_short('u')
                    .with_long("user")
                    .with_value_name("USER")
                    .with_help("The user who owns the repository."),
            )
            .with_arg(
                Arg::new()
                    .with_name("repository")
                    .with_short('r')
                    .with_long("repository")
                    .with_value_name("REPOSITORY")
                    .with_help("Name of the repository."),
            )
            .with_arg(
                Arg::new()
                    .with_name("output")
                    .with_short('o')
                    .with_long("output")
                    .with_value_name("OUTPUT")
                    .with_help("File path to save the json."),
            )
            .with_arg(
                Arg::new()
                    .with_name("display")
                    .with_short('d')
                    .with_long("display")
                    .with_help("Converts the json to an easier format (will remove some data)."),
            ),
        Command::new("releases", "Gives information on github releases.")
            .with_arg(
                Arg::new()
                    .with_name("user")
                    .with_short('u')
                    .with_long("user")
                    .with_value_name("USER")
                    .with_help("The user who owns the repository."),
            )
            .with_arg(
                Arg::new()
                    .with_name("repository")
                    .with_short('r')
                    .with_long("repository")
                    .with_value_name("REPOSITORY")
                    .with_help("Name of the repository."),
            )
            .with_arg(
                Arg::new()
                    .with_name("individual")
                    .with_short('i')
                    .with_long("individual")
                    .with_help("Downloads per release."),
            )
            .with_arg(
                Arg::new()
                    .with_name("link")
                    .with_short('l')
                    .with_long("link")
                    .with_help("Download links for releases (if not individual then for latest)."),
            )
            .with_arg(
                Arg::new()
                    .with_name("output")
                    .with_short('o')
                    .with_long("output")
                    .with_value_name("OUTPUT")
                    .with_help("File path to save the json."),
            )
            .with_arg(
                Arg::new()
                    .with_name("all")
                    .with_short('a')
                    .with_long("all")
                    .with_help("All json from request."),
            )
            .with_arg(
                Arg::new()
                    .with_name("display")
                    .with_short('d')
                    .with_long("display")
                    .with_help("Converts the json to an easier format (will remove some data)."),
            ),
        Command::new("user", "Gives information about a github user.")
            .with_arg(
                Arg::new()
                    .with_name("user")
                    .with_short('u')
                    .with_long("user")
                    .with_value_name("USER")
                    .with_help("The user you want information on."),
            )
            .with_arg(
                Arg::new()
                    .with_name("output")
                    .with_short('o')
                    .with_long("output")
                    .with_value_name("OUTPUT")
                    .with_help("File path to save the json."),
            )
            .with_arg(
                Arg::new()
                    .with_name("display")
                    .with_short('d')
                    .with_long("display")
                    .with_help("Converts the json to an easier format (will remove some data)."),
            ),
        Command::new("help", "Helps you with the commands.").with_short('h'),
    ]);

    let command = cli.match_commands();

    match command.name {
        "version" => cli.version(),
        "install" => {
            let os = match std::env::consts::OS {
                "windows" => OS::Windows,
                "macos" => OS::Mac,
                _ => panic!("Unsupported OS"),
            };

            install(&os);
        }
        "all" => {
            let user = command.get_value_of("user").throw_if_none();
            let repo = command.get_value_of("repository").throw_if_none();
            let output = command.get_value_of("output").to_option();
            let display = command.has("display");

            let output = output_to_path(output);

            all_command(user, repo, output, display);
        }
        "releases" => {
            let user = command.get_value_of("user").throw_if_none();
            let repo = command.get_value_of("repository").throw_if_none();
            let individual = command.has("individual");
            let link = command.has("link");
            let output = command.get_value_of("output").to_option();
            let all = command.has("all");
            let display = command.has("display");

            let output = output_to_path(output);

            releases_command(user, repo, individual, link, output, all, display);
        }
        "user" => {
            let user = command.get_value_of("user").throw_if_none();
            let output = command.get_value_of("output").to_option();
            let display = command.has("display");

            let output = output_to_path(output);

            user_command(user, output, display);
        }
        "help" => cli.help(),
        _ => cli.help(),
    }
}

fn output_to_path(output: Option<String>) -> Option<std::path::PathBuf> {
    match output {
        Some(path) => match validate_and_convert_path(path) {
            Ok(real_path) => Some(real_path),
            Err(err) => {
                println!("{}", err);
                std::process::exit(0)
            }
        },
        None => None,
    }
}
