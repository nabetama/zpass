use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input, MultiSelect, Password};

use crate::{item::Item, storage::Storage};

pub fn update(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let theme = ColorfulTheme::default();
    let mut storage = Storage::new(filename)?;

    let index = FuzzySelect::with_theme(&theme)
        .with_prompt("Which item would you like to update?: ")
        .default(0)
        .items(&storage.items[..])
        .interact()?;

    let item = &storage.items[index];

    let mut title = String::new();
    let mut username = String::new();
    let mut password = String::new();
    let mut website_url = String::new();

    let update_fields = MultiSelect::with_theme(&theme)
        .with_prompt("Which fields would you like to update?: ")
        .items(&["Title", "Username", "Password", "Website URL"])
        .interact()?;

    // TODO: expadability
    for field in update_fields {
        match field {
            0 => {
                title = Input::<String>::with_theme(&theme)
                    .with_prompt("Enter item title")
                    .allow_empty(false)
                    .default(item.title.to_string())
                    .interact_text()?;
            }
            1 => {
                username = Input::<String>::with_theme(&theme)
                    .with_prompt("Enter your username ")
                    .allow_empty(true)
                    .interact_text()?;
            }
            2 => {
                password = Password::with_theme(&theme)
                    .with_prompt("Password")
                    .with_confirmation("Repeat password", "Error: the passwords do not match.")
                    .allow_empty_password(true)
                    .interact()?;
            }
            3 => {
                website_url = Input::<String>::with_theme(&theme)
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

    storage.items[index] = Item::new(&title, &username, &password, &website_url);
    Ok(storage.save()?)
}
