use crate::entry::{self, Entry, EntryFlow};

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
