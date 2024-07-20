mod cmd_create;
mod cmd_remove;
mod cmd_search;
mod cmd_update;
mod item;
mod operations;
mod storage;

use std::env;

use dialoguer::{theme::ColorfulTheme, Select};

use cmd_create::create;
use cmd_remove::remove;
use cmd_search::search;
use cmd_update::update;
use operations::Operation;

const PASSWORDS_FILE: &str = ".zpass.json";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let theme = ColorfulTheme::default();

    let mut filename = String::new();
    let mut exist_current_passwords = false;

    if let Ok(current_dir) = env::current_dir() {
        let path = current_dir.join(PASSWORDS_FILE);
        filename = path.to_str().unwrap_or("").to_string();
        exist_current_passwords = true;
    }

    if !exist_current_passwords {
        let home = dirs::home_dir();
        if let Some(home_dir) = home {
            let path = home_dir.join(PASSWORDS_FILE);
            filename = path.to_str().unwrap_or("").to_string();
        }
        println!(
            "No password file found. Creating a new one at: {}",
            filename
        );
    }

    let items = vec![
        Operation::Search,
        Operation::Create,
        Operation::Update,
        Operation::Remove,
    ];

    let selection = Select::with_theme(&theme)
        .with_prompt("Pick your operation")
        .default(0)
        .items(&items)
        .interact()?;
    println!("You have selected: {}", items[selection]);

    let mut storage = storage::Storage::new(&filename)?;

    match items[selection] {
        Operation::Search => search(storage),
        Operation::Create => create(&mut storage),
        Operation::Update => update(&mut storage),
        Operation::Remove => remove(&mut storage),
    }
}
