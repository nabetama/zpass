use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub title: String,
    pub username: String,
    pub password: String,
    pub website_url: String,
}

impl Item {
    pub fn new(title: String, username: String, password: String, website_url: String) -> Self {
        Self {
            title,
            username,
            password,
            website_url,
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
