use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input, MultiSelect, Password};

use crate::{item::Item, storage::Storage};

pub fn update(storage: &mut Storage) -> Result<(), Box<dyn std::error::Error>> {
    let theme = ColorfulTheme::default();

    let index = FuzzySelect::with_theme(&theme)
        .with_prompt("Which item would you like to update?: ")
        .default(0)
        .items(&storage.items[..])
        .interact()?;

    let item = &storage.items[index];

    let mut new_item = Item::new(
        &item.title,
        &item.username,
        &item.password,
        &item.website_url,
    );

    let update_fields = MultiSelect::with_theme(&theme)
        .with_prompt("Which fields would you like to update?: ")
        .items(&["Title", "Username", "Password", "Website URL"])
        .interact()?;

    // TODO: expadability
    for field in update_fields {
        match field {
            0 => {
                let title = Input::<String>::with_theme(&theme)
                    .with_prompt("Enter item title")
                    .allow_empty(false)
                    .default(item.title.to_string())
                    .interact_text()?;
                new_item.title = title;
            }
            1 => {
                let username = Input::<String>::with_theme(&theme)
                    .with_prompt("Enter your username ")
                    .allow_empty(true)
                    .interact_text()?;
                new_item.username = username;
            }
            2 => {
                let password = Password::with_theme(&theme)
                    .with_prompt("Password")
                    .with_confirmation("Repeat password", "Error: the passwords do not match.")
                    .allow_empty_password(true)
                    .interact()?;
                new_item.password = password;
            }
            3 => {
                let website_url = Input::<String>::with_theme(&theme)
                    .with_prompt("Enter the website URL")
                    .validate_with(|input: &String| -> Result<(), &str> {
                        if input.starts_with("http") {
                            Ok(())
                        } else {
                            Err("The URL must start with http or https")
                        }
                    })
                    .default(item.website_url.clone())
                    .interact_text()?;
                new_item.website_url = website_url;
            }
            _ => unreachable!(),
        }
    }

    storage.items[index] = new_item;
    Ok(storage.save()?)
}
