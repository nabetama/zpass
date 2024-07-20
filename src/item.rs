use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub title: String,
    pub username: String,
    pub password: String,
    pub website_url: String,
    pub updated_at: Option<String>,
}

impl Item {
    pub fn new(title: &str, username: &str, password: &str, website_url: &str) -> Self {
        Self {
            title: title.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            website_url: website_url.to_string(),
            updated_at: Some(Local::now().format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Title: {}, Username: {}, Password: {} Website URL: {}",
            self.title, self.username, self.password, self.website_url
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_item() {
        let title = "title";
        let username = "username";
        let password = "password";
        let website_url = "website_url";

        let item = Item::new(title, username, password, website_url);

        assert_eq!(item.title, title);
        assert_eq!(item.username, username);
        assert_eq!(item.password, password);
        assert_eq!(item.website_url, website_url);
    }

    #[test]
    fn test_display() {
        let title = "title";
        let username = "username";
        let password = "password";
        let website_url = "website_url";

        let item = Item::new(title, username, password, website_url);

        let expected = format!(
            "Title: {}, Username: {}, Password: {} Website URL: {}",
            title, username, password, website_url
        );

        assert_eq!(item.to_string(), expected);
    }
}
