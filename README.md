# Github Stats (gstats)

`gstats` is a CLI tool that provides comprehensive statistics and information about GitHub repositories and users.

## Installation

`gstats` can be installed either by cloning the repository and building from source or by downloading a precompiled executable from the release page. Below are the instructions for both methods:

### Installing from Source

1. **Clone the Repository**:
   Clone `gstats` from GitHub to your local machine using the following command:

   ```bash
   git clone https://github.com/cqb13/github-stats
   cd gstats
   ```

2. **Build and Install with Cargo** (requires Rust's cargo tool):
   If you have Rust and Cargo installed, you can directly build and install the application using:

   ```bash
   cargo build --release
   ./target/release/gstats install
   ```

### Installing from Precompiled Executables

1. **Download the Latest Release**:
   Go to the [Releases page](rhttps://github.com/cqb13/github-stats/releases) of the `gstats` repository and download the appropriate executable for your operating system.

2. **Run Install Command**:
   After downloading, you need to run the installation command. Assuming you have downloaded `gstats.exe`, you can install it by navigating to the download location and running:

   ```bash
   ./gstats install
   ```

   This command will set up `gstats` on your system, making it ready for use.

### Post-Installation

After installing `gstats`, you can run `gstats help` to see all available commands and how to use them. Make sure that the installation path of `gstats` is added to your system's PATH, so it can be run from any terminal session.

## Usage

```bash
USAGE:
    github-stats [COMMAND] [OPTIONS]

COMMANDS:
    version      -v
        Displays the current version of github-stats
    install      -
        Installs the files and directories
    repo          -
        Gives all stats found on a repository as json
        -u           --user       <USER>       The user who owns the repository
        -r           --repository <REPOSITORY> Name of the repository
        -o           --output     <OUTPUT>     File path to save the json
        -d           --display    <>           Converts the json to an easier format (will remove some data)
    releases     -
        Gives information on github releases
        -u           --user       <USER>       The user who owns the repository
        -r           --repository <REPOSITORY> Name of the repository
        -i           --individual <>           Downloads per release
        -l           --link       <>           Download links for releases (if not individual then for latest)
        -o           --output     <OUTPUT>     File path to save the json
        -a           --all        <>           All json from request
        -d           --display    <>           Converts the json to an easier format (will remove some data)
    user         -
        Gives information about a github user
        -u           --user       <USER>       The user you want information on
        -o           --output     <OUTPUT>     File path to save the json
        -d           --display    <>           Converts the json to an easier format (will remove some data)
    followers    -
        Lists the followers of a github user
        -u           --user       <USER>       The user you want information on
        -t           --total      <TOTAL>      Gives the follower count
        -o           --output     <OUTPUT>     File path to save the json
        -d           --display    <>           Converts the json to an easier format (will remove some data)
    following    -
        Lists users the user is following
        -u           --user       <USER>       The user you want information on
        -t           --total      <TOTAL>      Gives the following count
        -o           --output     <OUTPUT>     File path to save the json
        -d           --display    <>           Converts the json to an easier format (will remove some data)
    help         -h
```

## Contributing

Contributions are welcome! Feel free to fork this repository and submit pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
