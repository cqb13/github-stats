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

## Features

- **Version**: Display the current version of `gstats`.
- **Install**: Installs the necessary files and directories for the tool.
- **All Stats**: Fetches all available statistics for a specified repository in JSON format.
- **Releases**: Provides information about the releases of a specified repository.
- **User Info**: Retrieves detailed information about a GitHub user.

## Usage

Below are the commands available in `gstats`:

### General

- `gstats help`: Displays help information about the commands.

### Version

- `gstats version`: Shows the current version of `gstats`.

### Repository Statistics

- `gstats all -u <USER> -r <REPOSITORY> [-o <OUTPUT>] [-d]`
  - `-u, --user USER`: The owner of the repository.
  - `-r, --repository REPOSITORY`: The name of the repository.
  - `-o, --output OUTPUT`: Optional. File path to save the JSON output.
  - `-d, --display`: Optional. Converts JSON to a more readable format (some data may be omitted).

### Release Information

- `gstats releases -u <USER> -r <REPOSITORY> [-i] [-l] [-o <OUTPUT>] [-a] [-d]`
  - `-u, --user USER`: The owner of the repository.
  - `-r, --repository REPOSITORY`: The name of the repository.
  - `-i, --individual`: Optional. Includes download counts per release.
  - `-l, --link`: Optional. Provides download links for releases.
  - `-o, --output OUTPUT`: Optional. File path to save the JSON output.
  - `-a, --all`: Optional. Fetches all JSON data from the request.
  - `-d, --display`: Optional. Converts JSON to a more readable format (some data may be omitted).

### User Information

- `gstats user -u <USER> [-o <OUTPUT>] [-d]`
  - `-u, --user USER`: The GitHub username to retrieve information for.
  - `-o, --output OUTPUT`: Optional. File path to save the JSON output.
  - `-d, --display`: Optional. Converts JSON to a more readable format (some data may be omitted).

## Contributing

Contributions are welcome! Feel free to fork this repository and submit pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
