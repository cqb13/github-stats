pub mod cli;

use cli::{Arg, Cli, Command};

fn main() {
    let cli = Cli::new().with_default_command("help").with_commands(vec![
        Command::new("version", "Displays the current version of github-stats").with_short('v'),
        Command::new("install", "Installs the files and directories"),
        Command::new("all", "Gives all stats found on a repository as json")
            .with_arg(
                Arg::new()
                    .with_name("owner")
                    .with_short('o')
                    .with_long("owner")
                    .with_value_name("OWNER")
                    .with_help("Owner of the repository"),
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
                    .with_short('f')
                    .with_long("output")
                    .with_value_name("OUTPUT")
                    .with_help("File path to save the json"),
            )
            .with_arg(
                Arg::new()
                    .with_name("display")
                    .with_short('d')
                    .with_long("display")
                    .with_help("Converts the json to an easier format (will remove some data)."),
            ),
        Command::new("downloads", "Gives download count of releases as json")
            .with_arg(
                Arg::new()
                    .with_name("owner")
                    .with_short('o')
                    .with_long("owner")
                    .with_value_name("OWNER")
                    .with_help("Owner of the repository"),
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
                    .with_short('f')
                    .with_long("output")
                    .with_value_name("OUTPUT")
                    .with_help("File path to save the json"),
            )
            .with_arg(
                Arg::new()
                    .with_name("display")
                    .with_short('d')
                    .with_long("display")
                    .with_help("Converts the json to an easier format (will remove some data)."),
            ),
        Command::new(
            "releases",
            "Gives names and download links for all releases as json",
        )
        .with_arg(
            Arg::new()
                .with_name("owner")
                .with_short('o')
                .with_long("owner")
                .with_value_name("OWNER")
                .with_help("Owner of the repository"),
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
                .with_short('f')
                .with_long("output")
                .with_value_name("OUTPUT")
                .with_help("File path to save the json"),
        )
        .with_arg(
            Arg::new()
                .with_name("display")
                .with_short('d')
                .with_long("display")
                .with_help("Converts the json to an easier format (will remove some data)."),
        ),
        Command::new("help", "Helps you with the commands").with_short('h'),
    ]);

    let command = cli.match_commands();

    match command.name {
        "version" => cli.version(),
        "all" => {
            let owner = command.get_value_of("owner").throw_if_none();
            let repo = command.get_value_of("repository").throw_if_none();
            let output = command.get_value_of("output").to_option();
            let display = command.has("display");
        }
        "downloads" => {
            let owner = command.get_value_of("owner").throw_if_none();
            let repo = command.get_value_of("repository").throw_if_none();
            let individual = command.has("individual");
            let link = command.has("link");
            let output = command.get_value_of("output").to_option();
            let display = command.has("display");
        }
        "releases" => {
            let owner = command.get_value_of("owner").throw_if_none();
            let repo = command.get_value_of("repository").throw_if_none();
            let output = command.get_value_of("output").to_option();
            let display = command.has("display");
        }
        "help" => cli.help(),
        _ => cli.help(),
    }
}
