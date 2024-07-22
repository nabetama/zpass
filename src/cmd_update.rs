use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input, MultiSelect, Password};

use crate::{item::Item, storage::Storage};

pub fn update(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let theme = ColorfulTheme::default();
    let mut storage = Storage::new(filename)?;

    if storage.items.is_empty() {
        println!("No items found. Please create some passwords first.");
        return Ok(());
    }

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

    // TODO: expandability
    for field in update_fields {
        match field {
            0 => {
                new_item.title = Input::<String>::with_theme(&theme)
                    .with_prompt("Enter item title")
                    .allow_empty(false)
                    .default(item.title.to_string())
                    .interact_text()?;
            }
            1 => {
                new_item.username = Input::<String>::with_theme(&theme)
                    .with_prompt("Enter your username ")
                    .allow_empty(true)
                    .interact_text()?;
            }
            2 => {
                new_item.password = Password::with_theme(&theme)
                    .with_prompt("Password")
                    .with_confirmation("Repeat password", "Error: the passwords do not match.")
                    .allow_empty_password(true)
                    .interact()?;
            }
            3 => {
                new_item.website_url = Input::<String>::with_theme(&theme)
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
            }
            _ => unreachable!(),
        }
    }

    storage.items[index] = new_item;
    Ok(storage.save()?)
}
