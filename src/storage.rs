use std::{
    fs::{remove_file, rename, OpenOptions},
    io::{self, BufReader},
};

use serde::{Deserialize, Serialize};

use crate::item::Item;

#[derive(Debug, Serialize, Deserialize)]
pub struct Storage {
    pub items: Vec<Item>,
    metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    path: String,
}

impl Storage {
    pub fn new(filename: &str) -> Result<Storage, Box<dyn std::error::Error>> {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(filename)?;

        if file.metadata()?.len() == 0 {
            return Ok(Self {
                items: Vec::new(),
                metadata: Metadata {
                    path: filename.to_string(),
                },
            });
        }

        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }

    fn write(&mut self) -> io::Result<()> {
        let tmp_file = format!("{}.tmp", self.metadata.path);
        let w = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&tmp_file)?;

        serde_json::to_writer_pretty(w, self)?;
        remove_file(&self.metadata.path)?;
        rename(tmp_file, &self.metadata.path)
    }

    pub fn save(&mut self) -> io::Result<()> {
        self.write()
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }
}
