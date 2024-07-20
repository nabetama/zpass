use console::style;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

use crate::storage::Storage;

pub fn search() -> Result<(), Box<dyn std::error::Error>> {
    let theme = ColorfulTheme::default();

    // storage から items をすべて取得する
    let items = Storage::new()?.items;
    // 取得したitems に対して、テキストを入力してインクリメンタルサーチする
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
