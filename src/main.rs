use std::fs::File;
use std::io::copy;
use thiserror::Error;
use dirs::download_dir; // For getting the user's Downloads directory

/// Custom error enum to handle IO and HTTP errors.
#[derive(Debug, Error)]
enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("HTTP error: {0}")]
    HttpRequest(#[from] reqwest::Error),
}

/// Main function to download a file from a URL and save it to the user's Downloads directory on Windows.
/// It first fetches the file from the URL, then saves it in the Downloads folder.
///
/// # Errors
///
/// This function can return:
/// - `MyError::Io` for any file-related errors (like creating or writing to the file).
/// - `MyError::HttpRequest` if the HTTP request fails.
///
/// # Platform-specific behavior
///
/// On Windows, the file is saved in the user's Downloads folder.
///
/// # Example
///
/// ```no_run
/// async fn main() -> Result<(), MyError> {
///     // This will download the Rust logo to the Downloads folder
///     download_file().await?;
///     Ok(())
/// }
/// ```
#[tokio::main]
async fn main() -> Result<(), MyError> {

    // The target URL to download the file from
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.jpg";

    // Perform an HTTP GET request to fetch the file
    let response = reqwest::get(target).await?;

    // Extract the file name from the URL or use a default name if extraction fails
    let file_name = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("downloaded_file.bin");

    println!("File to download: '{}'", file_name);

    // Get the user's Downloads directory on Windows
    let mut download_path = download_dir()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Downloads directory not found"))?;

    // Construct the full path to save the file in the Downloads directory
    download_path.push(file_name);
    println!("File will be saved under: '{:?}'", download_path);

    // Create the file in the Downloads directory
    let mut dest = File::create(&download_path)?;

    // Get the file content as text and copy it into the destination file
    let content = response.text().await?;
    copy(&mut content.as_bytes(), &mut dest)?;

    println!("Download completed successfully.");

    Ok(())
}
