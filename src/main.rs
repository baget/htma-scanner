mod shows;

use crate::shows::Category;
use crate::shows::{Show, get_shows_by_category};
use anyhow::Result;
use once_cell::sync::Lazy;
use std::collections::HashMap;

const FILE_NAME: &str = "shows.json";

// Create a static HashMap that's initialized on first access
static ENDPOINT_URLS: Lazy<HashMap<Category, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        Category::Comedy,
        "https://htma.smarticket.co.il/%D7%91%D7%99%D7%93%D7%95%D7%A8",
    );
    map.insert(
        Category::Music,
        "https://htma.smarticket.co.il/%D7%9E%D7%95%D7%A1%D7%99%D7%A7%D7%94",
    );
    map
});

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

    // Print only different shows
    let mut new_shows = current_shows.clone();
    new_shows.retain(|new_show| !prev_shows_vec.iter().any(|old_show| old_show == new_show));
    if new_shows.is_empty() {
        println!("No new show found.");
        return Ok(());
    }
    println!("New shows found:");
    for show in &new_shows {
        println!("{}", show);
    }

    export_file(&current_shows)?;

    Ok(())
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
    println!("Saved to {}", FILE_NAME);
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
