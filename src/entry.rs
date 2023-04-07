#[derive(Debug)]
pub enum Value {
    In(i32),
    Out(i32),
}

#[derive(Debug)]
pub struct Entry {
    pub date: String,
    pub desc: String,
    pub value: Value,
}

impl Entry {
    pub fn new(desc: &str, value: Value) -> Entry {
        Entry {
            date: "curr_date".to_string(),
            desc: desc.to_string(),
            value,
        }
    }
}
