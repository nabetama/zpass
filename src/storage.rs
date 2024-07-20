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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;

    #[test]
    fn test_storage_new() {
        let filename = "test_storage_new.json";
        let storage = Storage::new(filename).unwrap();
        assert_eq!(storage.items.len(), 0);
        remove_file(filename).unwrap();
    }

    #[test]
    fn test_storage_save() {
        let filename = "test_storage_save.json";
        let mut storage = Storage::new(filename).unwrap();
        storage.add_item(Item::new("title", "username", "password", "website"));
        storage.save().unwrap();
        let storage = Storage::new(filename).unwrap();
        assert_eq!(storage.items.len(), 1);
        remove_file(filename).unwrap();
    }

    #[test]
    fn test_storage_add_item() {
        let filename = "test_storage_add_item.json";
        let mut storage = Storage::new(filename).unwrap();
        storage.add_item(Item::new("title", "username", "password", "website"));
        assert_eq!(storage.items.len(), 1);
        remove_file(filename).unwrap();
    }
}
