use console::style;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

use crate::storage::Storage;

pub fn search(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let theme = ColorfulTheme::default();
    let storage = Storage::new(filename)?;

    let selection = FuzzySelect::with_theme(&theme)
        .with_prompt("Search: ")
        .default(0)
        .items(&storage.items)
        .interact()?;

    let item = &storage.items[selection];

    println!(
        "Title: {}\nUsername: {}\nPassword: {}\nWebsite URL: {}\nUpdated at: {}",
        item.title,
        style(item.username.to_string()).bold().green(),
        style(item.password.to_string()).bold().magenta(),
        item.website_url,
        item.updated_at.as_ref().unwrap_or(&"N/A".to_string())
    );

    Ok(())
}
