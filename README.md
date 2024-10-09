# File Downloader

This Rust project provides an asynchronous command-line tool to download files from the web and save them directly to the user's `Downloads` directory (on Windows). The tool leverages the `reqwest` library for making HTTP requests and uses the `dirs` crate to determine the path to the user's `Downloads` folder. The downloaded file is saved using the file name extracted from the URL.

## Features

- Downloads a file from a given URL.
- Saves the file in the user's `Downloads` directory (Windows specific).
- Extracts the file name from the URL, or defaults to `downloaded_file.bin` if not available.
- Custom error handling using the `thiserror` crate to provide meaningful error messages for I/O and HTTP errors.

## Requirements

- **Rust**: Make sure you have Rust installed. If not, you can get it from [rust-lang.org](https://www.rust-lang.org/).
- **Crates**:
    - `tokio`: For asynchronous runtime.
    - `reqwest`: For making HTTP requests.
    - `dirs`: To locate the user's `Downloads` folder.
    - `thiserror`: For custom error handling.

## Installation

1. Clone the repository:

    ```bash
    git remote add origin git@github.com:mehdijafarzadeh/rust_file_downloader.git
    ```

2. Navigate to the project directory:

    ```bash
    cd rust_file_downloader
    ```

3. Build the project:

    ```bash
    cargo build 
    ```

## Usage

### Run the Program

To run the file downloader, simply execute the following:

```bash
cargo run
