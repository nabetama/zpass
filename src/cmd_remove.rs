use dialoguer::{theme::ColorfulTheme, FuzzySelect};

use crate::storage::Storage;

pub fn remove() -> Result<(), Box<dyn std::error::Error>> {
    let theme = ColorfulTheme::default();

    let mut storage = Storage::new()?;
    let index = FuzzySelect::with_theme(&theme)
        .with_prompt("Which item would you like to remove?: ")
        .default(0)
        .items(&storage.items[..])
        .interact()?;

    storage.items.remove(index);
    Ok(storage.save()?)
}