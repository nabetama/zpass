use console::style;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

use crate::storage::Storage;

pub fn search(storage: Storage) -> Result<(), Box<dyn std::error::Error>> {
    let theme = ColorfulTheme::default();
    let items = storage.items;

    let selection = FuzzySelect::with_theme(&theme)
        .with_prompt("Search: ")
        .default(0)
        .items(&items[..])
        .interact()?;

    let item = &items[selection];

    println!(
        "Title: {}\nUsername: {}\nPassword: {}\nWebsite URL: {}",
        item.title,
        style(item.username.to_string()).bold().green(),
        style(item.password.to_string()).bold().red(),
        item.website_url,
    );

    Ok(())
}
