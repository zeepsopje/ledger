use std::{fs::{self, File}, io::Read};
use crate::entry::{self, Entry, EntryFlow};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Ledger {
    pub balance: i32,
    pub entries: Vec<Entry>,
}

impl Ledger {
    pub fn new() -> Ledger {
        Ledger {
            balance: 0,
            entries: Vec::new(),
        }
    }

    pub fn from_file(path: &str) -> Option<Ledger> {
        match File::open(path) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap_or_else(|_| panic!("Corrupt config file"));
                let ledger: Ledger = serde_json::from_str(contents.as_str()).unwrap();
                Some(ledger)
            },
            Err(_) => None,
        }
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let serialized = serde_json::to_string(self)?;
        fs::write(path, serialized)?;
        Ok(())
    }

    fn add_entry(&mut self, entry: Entry) {
        match entry.flow {
            entry::EntryFlow::In => self.balance += entry.value,
            entry::EntryFlow::Out => self.balance -= entry.value,
        }

        self.entries.push(entry);
    }

    pub fn withdraw(&mut self, desc: &str, value: i32) {
        self.add_entry(Entry::new(desc, value, EntryFlow::Out));
    }

    pub fn deposit(&mut self, desc: &str, value: i32) {
        self.add_entry(Entry::new(desc, value, EntryFlow::In));
    }
}
