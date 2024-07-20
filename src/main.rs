mod cmd_create;
mod cmd_remove;
mod cmd_search;
mod cmd_update;
mod item;
mod storage;

use dialoguer::{theme::ColorfulTheme, Select};

use cmd_create::create;
use cmd_remove::remove;
use cmd_search::search;
use cmd_update::update;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let theme = ColorfulTheme::default();

    let items = vec!["Search", "Create", "Update", "Remove"];

    let selection = Select::with_theme(&theme)
        .with_prompt("Pick your operation")
        .default(0)
        .items(&items)
        .interact()?;
    println!("You have selected: {}", items[selection]);

    match items[selection] {
        "search" => search(),
        "create" => create(),
        "update" => update(),
        "delete" => remove(),
        _ => unreachable!(),
    }
}
