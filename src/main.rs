mod shows;

use crate::shows::Category;
use crate::shows::get_shows_by_category;
use anyhow::Result;
use once_cell::sync::Lazy;
use std::collections::HashMap;

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

    let mut shows_vec = get_shows_by_category(Category::Comedy)?;
    let music_vec = get_shows_by_category(Category::Music)?;

    shows_vec.extend(music_vec);
    println!("{:#?}", shows_vec);

    Ok(())
}
