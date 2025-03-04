pub mod cli;
pub mod commands;
pub mod styles;
pub mod utils;

<<<<<<< HEAD
use crate::cli::{Arg, Cli, CmdOption, Command};
=======
use crate::cli::{Arg, Cli, Command};
>>>>>>> refs/remotes/origin/main
use crate::commands::relations::{relations_command, RelationType};
use crate::commands::releases::releases_command;
use crate::commands::repo::repo_command;
use crate::commands::user::user_command;
use crate::utils::{install, validate_and_convert_path, OS};

fn main() {
<<<<<<< HEAD
    let cli = Cli::new()
        .with_command(Command::new("help", "Prints help information").with_option(
            CmdOption::new("command", "COMMAND", "The command you want help with").optional(),
        ))
        .with_command(Command::new("version", "Prints version information"))
        .with_command(Command::new(
            "install",
            "Installs the files and directories",
        ))
        .with_command(
            Command::new("repo", "Displays general information about a repositoy")
                .with_option(CmdOption::new(
                    "user",
                    "USER",
                    "The user who owns the repository",
                ))
                .with_option(CmdOption::new(
                    "repository",
                    "REPOSITORY",
                    "The name of the repository",
                ))
                .with_arg(
                    Arg::new("output", "File path to save the json", "output", 'o')
                        .with_value_name("OUTPUT"),
                )
                .with_arg(Arg::new(
                    "display",
                    "Converts the json to an easier format (will remove some data)",
                    "display",
                    'd',
                )),
        )
        .with_command(
            Command::new("repo", "Displays general information about a repositoy")
                .with_option(CmdOption::new(
                    "user",
                    "USER",
                    "The user who owns the repository",
                ))
                .with_option(CmdOption::new(
                    "repository",
                    "REPOSITORY",
                    "The name of the repository",
                ))
                .with_arg(
                    Arg::new("output", "File path to save the json", "output", 'o')
                        .with_value_name("OUTPUT"),
                )
                .with_arg(Arg::new(
                    "display",
                    "Converts the json to an easier format (will remove some data)",
                    "display",
                    'd',
                )),
        )
        .with_command(
            Command::new("releases", "Displays informatiom about github releases")
                .with_option(CmdOption::new(
                    "user",
                    "USER",
                    "The user who owns the repository",
                ))
                .with_option(CmdOption::new(
                    "repository",
                    "REPOSITORY",
                    "The name of the repository",
                ))
                .with_arg(Arg::new(
                    "individual",
                    "Displays downloads per release along with total downloads",
                    "individual",
                    'i',
                ))
                .with_arg(Arg::new(
                    "link",
                    "Displays download links for releases (if not individiual then for latest)",
                    "link",
                    'l',
                ))
                .with_arg(
                    Arg::new("output", "File path to save the json", "output", 'o')
                        .with_value_name("OUTPUT"),
                )
                .with_arg(Arg::new(
                    "all",
                    "Saves all the json from the request insteaad of a cleaned up version",
                    "all",
                    'a',
                ))
                .with_arg(Arg::new(
                    "display",
                    "Converts the json to an easier format (will remove some data)",
                    "display",
                    'd',
                )),
        )
        .with_command(
            Command::new("user", "Displays information about a github user")
                .with_option(CmdOption::new(
                    "user",
                    "USER",
                    "The user who owns the repository",
                ))
                .with_arg(
                    Arg::new("output", "File path to save the json", "output", 'o')
                        .with_value_name("OUTPUT"),
                )
                .with_arg(Arg::new(
                    "display",
                    "Converts the json to an easier format (will remove some data)",
                    "display",
                    'd',
                )),
        )
        .with_command(
            Command::new("followers", "Displays the followers of a github user")
                .with_option(CmdOption::new(
                    "user",
                    "USER",
                    "The user who owns the repository",
                ))
                .with_arg(Arg::new(
                    "total",
                    "Displays the total follower count",
                    "toal",
                    't',
                ))
                .with_arg(
                    Arg::new("output", "File path to save the json", "output", 'o')
                        .with_value_name("OUTPUT"),
                )
                .with_arg(Arg::new(
                    "display",
                    "Converts the json to an easier format (will remove some data)",
                    "display",
                    'd',
                )),
        )
        .with_command(
            Command::new("following", "Displays the users a github user is following")
                .with_option(CmdOption::new(
                    "user",
                    "USER",
                    "The user who owns the repository",
                ))
                .with_arg(Arg::new(
                    "total",
                    "Displays the total follower count",
                    "toal",
                    't',
                ))
                .with_arg(
                    Arg::new("output", "File path to save the json", "output", 'o')
                        .with_value_name("OUTPUT"),
                )
                .with_arg(Arg::new(
                    "display",
                    "Converts the json to an easier format (will remove some data)",
                    "display",
                    'd',
                )),
        );
=======
    let cli = Cli::new().with_default_command("help").with_commands(vec![
        Command::new("version", "Displays the current version of github-stats").with_short('v'),
        Command::new("install", "Installs the files and directories"),
        Command::new("repo", "Gives all stats found on a repository as json")
            .with_arg(
                Arg::new()
                    .with_name("user")
                    .with_short('u')
                    .with_long("user")
                    .with_value_name("USER")
                    .with_help("The user who owns the repository"),
            )
            .with_arg(
                Arg::new()
                    .with_name("repository")
                    .with_short('r')
                    .with_long("repository")
                    .with_value_name("REPOSITORY")
                    .with_help("Name of the repository"),
            )
            .with_arg(
                Arg::new()
                    .with_name("output")
                    .with_short('o')
                    .with_long("output")
                    .with_value_name("OUTPUT")
                    .with_help("File path to save the json"),
            )
            .with_arg(
                Arg::new()
                    .with_name("display")
                    .with_short('d')
                    .with_long("display")
                    .with_help("Converts the json to an easier format (will remove some data)"),
            ),
        Command::new("releases", "Gives information on github releases")
            .with_arg(
                Arg::new()
                    .with_name("user")
                    .with_short('u')
                    .with_long("user")
                    .with_value_name("USER")
                    .with_help("The user who owns the repository"),
            )
            .with_arg(
                Arg::new()
                    .with_name("repository")
                    .with_short('r')
                    .with_long("repository")
                    .with_value_name("REPOSITORY")
                    .with_help("Name of the repository"),
            )
            .with_arg(
                Arg::new()
                    .with_name("individual")
                    .with_short('i')
                    .with_long("individual")
                    .with_help("Downloads per release"),
            )
            .with_arg(
                Arg::new()
                    .with_name("link")
                    .with_short('l')
                    .with_long("link")
                    .with_help("Download links for releases (if not individual then for latest)"),
            )
            .with_arg(
                Arg::new()
                    .with_name("output")
                    .with_short('o')
                    .with_long("output")
                    .with_value_name("OUTPUT")
                    .with_help("File path to save the json"),
            )
            .with_arg(
                Arg::new()
                    .with_name("all")
                    .with_short('a')
                    .with_long("all")
                    .with_help("All json from request"),
            )
            .with_arg(
                Arg::new()
                    .with_name("display")
                    .with_short('d')
                    .with_long("display")
                    .with_help("Converts the json to an easier format (will remove some data)"),
            ),
        Command::new("user", "Gives information about a github user")
            .with_arg(
                Arg::new()
                    .with_name("user")
                    .with_short('u')
                    .with_long("user")
                    .with_value_name("USER")
                    .with_help("The user you want information on"),
            )
            .with_arg(
                Arg::new()
                    .with_name("output")
                    .with_short('o')
                    .with_long("output")
                    .with_value_name("OUTPUT")
                    .with_help("File path to save the json"),
            )
            .with_arg(
                Arg::new()
                    .with_name("display")
                    .with_short('d')
                    .with_long("display")
                    .with_help("Converts the json to an easier format (will remove some data)"),
            ),
        Command::new("followers", "Lists the followers of a github user")
            .with_arg(
                Arg::new()
                    .with_name("user")
                    .with_short('u')
                    .with_long("user")
                    .with_value_name("USER")
                    .with_help("The user you want information on"),
            )
            .with_arg(
                Arg::new()
                    .with_name("total")
                    .with_short('t')
                    .with_long("total")
                    .with_value_name("TOTAL")
                    .with_help("Gives the follower count"),
            )
            .with_arg(
                Arg::new()
                    .with_name("output")
                    .with_short('o')
                    .with_long("output")
                    .with_value_name("OUTPUT")
                    .with_help("File path to save the json"),
            )
            .with_arg(
                Arg::new()
                    .with_name("display")
                    .with_short('d')
                    .with_long("display")
                    .with_help("Converts the json to an easier format (will remove some data)"),
            ),
        Command::new("following", "Lists users the user is following")
            .with_arg(
                Arg::new()
                    .with_name("user")
                    .with_short('u')
                    .with_long("user")
                    .with_value_name("USER")
                    .with_help("The user you want information on"),
            )
            .with_arg(
                Arg::new()
                    .with_name("total")
                    .with_short('t')
                    .with_long("total")
                    .with_value_name("TOTAL")
                    .with_help("Gives the following count"),
            )
            .with_arg(
                Arg::new()
                    .with_name("output")
                    .with_short('o')
                    .with_long("output")
                    .with_value_name("OUTPUT")
                    .with_help("File path to save the json"),
            )
            .with_arg(
                Arg::new()
                    .with_name("display")
                    .with_short('d')
                    .with_long("display")
                    .with_help("Converts the json to an easier format (will remove some data)"),
            ),
        Command::new("help", "Helps you with the commands").with_short('h'),
    ]);
>>>>>>> refs/remotes/origin/main

    let command = cli.match_commands();

    match command.name.as_str() {
        "version" => cli.version(),
        "install" => {
            let os = match std::env::consts::OS {
                "windows" => OS::Windows,
                "macos" => OS::Mac,
                _ => panic!("Unsupported OS"),
            };

            install(&os);
        }
        "repo" => {
<<<<<<< HEAD
            let user = command.get_option("user").throw_if_none();
            let repo = command.get_option("repository").throw_if_none();
            let output = command.get_arg("output").to_option();
=======
            let user = command.get_value_of("user").throw_if_none();
            let repo = command.get_value_of("repository").throw_if_none();
            let output = command.get_value_of("output").to_option();
>>>>>>> refs/remotes/origin/main
            let display = command.has("display");

            let output = output_to_path(output);

            repo_command(user, repo, output, display);
        }
        "releases" => {
            let user = command.get_option("user").throw_if_none();
            let repo = command.get_option("repository").throw_if_none();
            let individual = command.has("individual");
            let link = command.has("link");
            let output = command.get_arg("output").to_option();
            let all = command.has("all");
            let display = command.has("display");

            let output = output_to_path(output);

            releases_command(user, repo, individual, link, output, all, display);
        }
        "user" => {
            let user = command.get_option("user").throw_if_none();
            let output = command.get_option("output").to_option();
            let display = command.has("display");

            let output = output_to_path(output);

            user_command(user, output, display);
        }
        "followers" => {
            let user = command.get_option("user").throw_if_none();
            let total = command.has("total");
            let output = command.get_arg("output").to_option();
            let display = command.has("display");

            let output = output_to_path(output);

            relations_command(user, total, output, display, RelationType::Follower);
        }
        "following" => {
            let user = command.get_option("user").throw_if_none();
            let total = command.has("total");
            let output = command.get_arg("output").to_option();
            let display = command.has("display");

            let output = output_to_path(output);

            relations_command(user, total, output, display, RelationType::Following);
        }
        "help" => {
            let command = command.get_option("command").to_option();
            cli.help(command.as_deref())
        }
        _ => cli.help(None),
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
