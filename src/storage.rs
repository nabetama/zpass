use std::{
    fs::{remove_file, rename, File, OpenOptions},
    io::{self, BufReader},
};

use serde::{Deserialize, Serialize};

use crate::item::Item;

#[derive(Debug, Serialize, Deserialize)]
pub struct Storage {
    pub items: Vec<Item>,
}

const STORAGE_FILE: &str = ".zpass.json";

impl Storage {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(STORAGE_FILE)?;

        if file.metadata()?.len() == 0 {
            return Ok(Self { items: Vec::new() });
        }

        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }

    fn write(&mut self) -> io::Result<()> {
        let tmp_file = format!("{}.tmp", STORAGE_FILE);
        let w = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&tmp_file)?;

        serde_json::to_writer_pretty(w, self)?;
        remove_file(STORAGE_FILE)?;
        rename(tmp_file, STORAGE_FILE)
    }

    pub fn save(&mut self) -> io::Result<()> {
        self.write()
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }
}
