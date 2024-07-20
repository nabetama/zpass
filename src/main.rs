mod cmd_create;
mod cmd_search;
mod item;
mod storage;

use dialoguer::{theme::ColorfulTheme, Select};

use cmd_create::create;
use cmd_search::search;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let theme = ColorfulTheme::default();

    let items = vec!["search", "create", "update", "delete"];

    let selection = Select::with_theme(&theme)
        .with_prompt("Pick your operation")
        .default(0)
        .items(&items)
        .interact()?;
    println!("You have selected: {}", items[selection]);

    match items[selection] {
        "search" => search(),
        "create" => create(),
        "update" => unimplemented!(),
        "delete" => unimplemented!(),
        _ => unreachable!(),
    }
}
