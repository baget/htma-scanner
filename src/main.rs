mod shows;

use crate::shows::Category;
use crate::shows::{Show, get_shows_by_category};
use anyhow::Result;
use std::env;
use urlencoding::encode;

const FILE_NAME: &str = "shows.json";

#[derive(Debug, thiserror::Error)]
enum HtmaError {
    #[error("Category not found")]
    CategoryNotFound,
}

fn main() -> Result<()> {
    println!("htma-scanner");

    let prev_shows_vec = import_file().unwrap_or_else(|_| {
        println!("No previous data found, fetching new shows...");
        vec![]
    });

    let current_shows = get_shows()?;

    // Check if the new shows are different from the previous ones
    if prev_shows_vec == current_shows {
        println!("No new show found.");
        return Ok(());
    }

    if let Some(new_shows) = check_for_new_shows(prev_shows_vec, &current_shows) {
        println!("New shows found:");
        for show in &new_shows {
            println!("{}", show);
        }

        let msg = format!(
            "*הופעות חדשות*:\n{}",
            new_shows
                .iter()
                .map(|s| format!("{} 🔛 `{} ({})`", s.title, s.date, s.time))
                .collect::<Vec<_>>()
                .join("\r\n")
        );
        notify(msg)?;

        // Export the new shows to a file
        export_file(&current_shows)?;
        println!("Saved to {}", FILE_NAME);
    } else {
        println!("No new show found.");
    }

    Ok(())
}

/// Compares two lists of shows and identifies new shows in the current list.
///
/// # Arguments
/// * `prev_shows_vec` - A vector of `Show` objects representing the previous list of shows.
/// * `current_shows` - A reference to a vector of `Show` objects representing the current list of shows.
///
/// # Returns
/// * `Option<Vec<Show>>` - Returns `Some(Vec<Show>)` containing the new shows if there are any, or `None` if no new shows are found.
///
/// # Behavior
/// * Retains only the shows in `current_shows` that are not present in `prev_shows_vec`.
/// * Returns `None` if no new shows are found.
fn check_for_new_shows(prev_shows_vec: Vec<Show>, current_shows: &Vec<Show>) -> Option<Vec<Show>> {
    // Print only different shows
    let mut new_shows = current_shows.clone();
    new_shows.retain(|new_show| !prev_shows_vec.iter().any(|old_show| old_show == new_show));
    if new_shows.is_empty() {
        return None;
    }

    Some(new_shows)
}

/// Retrieves a list of shows from multiple categories.
///
/// # Returns
/// * `Result<Vec<Show>>` - A vector of `Show` objects if successful, or an error if the operation fails.
///
/// # Errors
/// * Returns an error if fetching shows by category fails.
///
/// # Behavior
/// * Fetches shows from the `Comedy` and `Music` categories.
/// * Combines the results into a single vector.
/// * Sorts the shows by date and time in ascending order.
fn get_shows() -> Result<Vec<Show>> {
    let mut shows_vec = get_shows_by_category(Category::Comedy)?;
    let music_vec = get_shows_by_category(Category::Music)?;

    shows_vec.extend(music_vec);
    // Sort the shows by date and by time
    shows_vec.sort_by(|a, b| a.date.cmp(&b.date).then_with(|| a.time.cmp(&b.time)));
    Ok(shows_vec)
}

/// Exports a vector of `Show` objects to a JSON file.
///
/// # Arguments
/// * `shows_vec` - A reference to a vector of `Show` objects to be exported.
///
/// # Returns
/// * `Result<()>` - Returns `Ok(())` if the operation is successful, or an error if it fails.
///
/// # Errors
/// * Returns an error if serialization to JSON fails or if writing to the file system fails.
fn export_file(shows_vec: &Vec<Show>) -> Result<()> {
    // Convert to JSON
    let json = serde_json::to_string_pretty(&shows_vec)?;

    // Save to file
    std::fs::write(FILE_NAME, json)?;
    Ok(())
}

/// Imports a list of `Show` objects from a JSON file.
///
/// # Returns
/// * `Result<Vec<Show>>` - A vector of `Show` objects if successful, or an error if the operation fails.
///
/// # Errors
/// * Returns an error if reading the file or deserializing the JSON fails.
fn import_file() -> Result<Vec<Show>> {
    // Read from a file
    let json = std::fs::read_to_string(FILE_NAME)?;

    // Deserialize JSON to Vec<Show>
    let shows_vec: Vec<Show> = serde_json::from_str(&json)?;
    Ok(shows_vec)
}

/// Sends a notification message via Telegram.
///
/// # Arguments
/// * `text` - A `String` containing the message to be sent.
///
/// # Returns
/// * `Result<()>` - Returns `Ok(())` if the notification is sent successfully, or an error if it fails.
///
/// # Errors
/// * Returns an error if the `TELEGRAM_TOKEN` or `CHAT_ID` environment variables are not set.
/// * Returns an error if the HTTP request to the Telegram API fails.
///
/// # Behavior
/// * Encodes the message text to ensure it is URL-safe.
/// * Constructs the Telegram API URL using the bot token and chat ID.
/// * Sends the message using a blocking HTTP GET request.
fn notify(text: String) -> Result<()> {
    const TELEGRAM_BASE_URL: &str = "https://api.telegram.org";

    let encoded = encode(&text);

    let token = env::var("TELEGRAM_TOKEN")?;
    let chat_id = env::var("CHAT_ID")?;

    let url = format!(
        "{}/bot{}/sendMessage?chat_id={}&parse_mode=Markdown&text={}",
        TELEGRAM_BASE_URL, token, chat_id, encoded
    );

    let _resp = reqwest::blocking::get(&url)?;

    Ok(())
}
