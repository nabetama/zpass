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
