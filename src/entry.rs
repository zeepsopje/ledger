use serde::{Serialize, Deserialize};
use clap::{Args, ValueEnum};

#[derive(Debug, Args)]
pub struct EntryArg {
    /// Value of the entry
    pub value: i32,

    /// Description of the entry
    pub desc: String,
}

#[derive(Debug, ValueEnum, Clone, Serialize, Deserialize)]
pub enum EntryFlow {
    In,
    Out,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub date: String,
    pub desc: String,
    pub value: i32,
    pub flow: EntryFlow,
}

impl Entry {
    pub fn new(desc: &str, value: i32, flow: EntryFlow) -> Entry {
        Entry {
            date: "curr_date".to_string(),
            desc: desc.to_string(),
            value,
            flow,
        }
    }
}
