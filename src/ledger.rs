use crate::entry::{self, Entry};

#[derive(Debug)]
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

    fn add_entry(&mut self, entry: Entry) {
        match entry.value {
            entry::Value::In(value) => self.balance += value,
            entry::Value::Out(value) => self.balance -= value,
        }

        self.entries.push(entry);
    }

    pub fn withdraw(&mut self, desc: &str, value: i32) {
        self.add_entry(Entry::new(desc, entry::Value::Out(value)));
    }

    pub fn deposit(&mut self, desc: &str, value: i32) {
        self.add_entry(Entry::new(desc, entry::Value::In(value)));
    }
}
