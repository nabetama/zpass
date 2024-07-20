use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub title: String,
    pub username: String,
    pub password: String,
    pub website_url: String,
}

impl Item {
    pub fn new(title: &str, username: &str, password: &str, website_url: &str) -> Self {
        Self {
            title: title.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            website_url: website_url.to_string(),
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
