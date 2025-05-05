# HTMA Scanner

## Overview (English)

HTMA Scanner is a Rust-based project (Quick and Dirty) designed to scrape and parse show information from a HTMA
website (היכל התרבות - מעלה אדומים).

It retrieves data such as show titles, dates, times, and categories (e.g., Comedy, Music) by making HTTP requests and
parsing the HTML content.

### Features

- Fetches show data by category.
- Parses Hebrew date strings into Rust's `NaiveDate` format.
- Categorizes shows into predefined categories (e.g., Comedy, Music).
- Uses `reqwest` for HTTP requests and `scraper` for HTML parsing.

### Project Structure

- **`src/shows.rs`**: Contains the main logic for fetching and parsing show data.
- **Category Enum**: Defines categories like `Comedy` and `Music`.
- **Show Struct**: Represents a show with attributes like title, date, time, and category.

### Dependencies

- `reqwest`: For making HTTP requests.
- `scraper`: For parsing HTML content.
- `serde`: For serializing and deserializing data.
- `anyhow`: For error handling.

### How to Run

1. Clone the repository.
2. Ensure you have Rust and Cargo installed.
3. Run the project using:
   ```bash
   cargo run
   ```
4. The program will fetch and display show data for the specified category.
