use dialoguer::{theme::ColorfulTheme, FuzzySelect};

use crate::storage::Storage;

pub fn remove(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let theme = ColorfulTheme::default();
    let mut storage = Storage::new(filename)?;

    if storage.items.is_empty() {
        println!("No items found. Please create some passwords first.");
        return Ok(());
    }

    let selection = FuzzySelect::with_theme(&theme)
        .with_prompt("Which item would you like to remove?: ")
        .default(0)
        .items(&storage.items[..])
        .interact_opt()?;

    match selection {
        Some(index) => {
            storage.items.remove(index);
        }
        None => {
            println!("No item selected");
        }
    }

    Ok(storage.save()?)
}
