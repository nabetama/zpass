use dialoguer::{theme::ColorfulTheme, Input, Password};

use crate::{item::Item, storage::Storage};

pub fn create(storage: &mut Storage) -> Result<(), Box<dyn std::error::Error>> {
    let theme = ColorfulTheme::default();

    let title = Input::<String>::with_theme(&theme)
        .with_prompt("Enter item title")
        .allow_empty(false)
        .interact_text()?;

    let username = Input::<String>::with_theme(&theme)
        .with_prompt("Enter your username ")
        .allow_empty(true)
        .interact_text()?;

    let password = Password::with_theme(&theme)
        .with_prompt("Password")
        .with_confirmation("Repeat password", "Error: the passwords do not match.")
        .allow_empty_password(true)
        .interact()?;

    let website_url = Input::<String>::with_theme(&theme)
        .with_prompt("Enter the website URL")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.starts_with("http") {
                Ok(())
            } else {
                Err("The URL must start with http or https")
            }
        })
        .interact_text()?;

    storage.add_item(Item::new(&title, &username, &password, &website_url));
    storage.save()?;

    Ok(())
}
