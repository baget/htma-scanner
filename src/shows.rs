use crate::HtmaError;
use chrono::{NaiveDate, NaiveTime};
use once_cell::sync::Lazy;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;

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

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Serialize, Deserialize, PartialOrd)]
pub enum Category {
    None,
    Comedy,
    Music,
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::None => write!(f, "None"),
            Category::Comedy => write!(f, "Comedy"),
            Category::Music => write!(f, "Music"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Show {
    pub title: String,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub category: Category,
}

impl Show {
    pub fn default() -> Self {
        Show {
            title: String::new(),
            date: NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            time: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            category: Category::None,
        }
    }
}

impl Display for Show {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Title: {}, Date: {}, Time: {}, Category: {}",
            self.title, self.date, self.time, self.category
        )
    }
}

/// Parses a Hebrew date string into a `NaiveDate`.
///
/// # Arguments
/// * `date_str` - A string slice containing the Hebrew date in the format "day month, year".
///
/// # Returns
/// * `Ok(NaiveDate)` - The parsed date if successful.
/// * `Err(anyhow::Error)` - An error if the date format is invalid or parsing fails.
fn parse_hebrew_date(date_str: &str) -> anyhow::Result<NaiveDate> {
    let months = [
        "ינואר",
        "פברואר",
        "מרץ",
        "אפריל",
        "מאי",
        "יוני",
        "יולי",
        "אוגוסט",
        "ספטמבר",
        "אוקטובר",
        "נובמבר",
        "דצמבר",
    ];

    let parts: Vec<&str> = date_str.split(',').collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!("Invalid date format"));
    }

    let day_and_month: Vec<&str> = parts[1].trim().split_whitespace().collect();
    if day_and_month.len() != 3 {
        return Err(anyhow::anyhow!("Invalid date format"));
    }

    let day: u32 = day_and_month[0]
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid day"))?;
    let month = months
        .iter()
        .position(|&m| m == day_and_month[1])
        .ok_or_else(|| anyhow::anyhow!("Invalid month"))?
        + 1;
    let year: i32 = day_and_month[2]
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid year"))?;

    NaiveDate::from_ymd_opt(year, month as u32, day).ok_or_else(|| anyhow::anyhow!("Invalid date"))
}

/// Retrieves a list of shows for a given category by parsing the associated HTML.
///
/// # Arguments
/// * `category` - The category of shows to retrieve.
///
/// # Returns
/// * `Ok(Vec<Show>)` - A vector of `Show` objects if successful.
/// * `Err(Error)` - An error if the HTML parsing or data extraction fails.
pub fn get_shows_by_category(category: Category) -> anyhow::Result<Vec<Show>> {
    let mut ret_vec = Vec::new();

    let html = get_html_by_category(category)?;

    let document = Html::parse_document(&html);
    let selector = Selector::parse(r#"div[class="category_shows"]"#).unwrap();
    let shows_element = document.select(&selector).next().unwrap();

    let details_selector = Selector::parse(r#"div.details-container"#).unwrap();
    let details_element = shows_element.select(&details_selector);

    for element in details_element {
        let mut show = Show::default();
        show.category = category;

        if let Some(title) = element.select(&Selector::parse("h2").unwrap()).next() {
            show.title = title.text().collect::<String>().trim().to_string();
        }

        if let Some(date) = element
            .select(&Selector::parse("div.date_container").unwrap())
            .next()
        {
            let date_text = date.text().collect::<String>().trim().to_string();
            show.date = parse_hebrew_date(&date_text)?;
        }

        if let Some(time) = element
            .select(&Selector::parse("div.time_container").unwrap())
            .next()
        {
            let time_text = time.text().collect::<String>().trim().replace("בשעה ", "");
            show.time = NaiveTime::parse_from_str(&time_text, "%H:%M")?;
        }

        ret_vec.push(show);
    }
    Ok(ret_vec)
}

/// Retrieves the URL associated with the given `Category`.
///
/// # Arguments
/// * `category` - The category for which the URL is to be retrieved.
///
/// # Returns
/// * `Ok(&'static str)` - The URL as a static string if the category exists.
/// * `Err(Error)` - An error if the category is not found in the `ENDPOINT_URLS` map.
fn get_url(category: Category) -> anyhow::Result<&'static str> {
    ENDPOINT_URLS
        .get(&category)
        .map(|&url| url)
        .ok_or_else(|| HtmaError::CategoryNotFound.into())
}
/// Retrieves the HTML content for a given category by making an HTTP GET request.
///
/// # Arguments
/// * `category` - The category for which the HTML content is to be retrieved.
///
/// # Returns
/// * `Ok(String)` - The HTML content as a string if the request is successful.
/// * `Err(Error)` - An error if the URL retrieval or HTTP request fails.
fn get_html_by_category(category: Category) -> anyhow::Result<String> {
    let url = get_url(category)?;
    let body = reqwest::blocking::get(url)?.text()?;

    Ok(body)
}
