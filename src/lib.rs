use std::collections::HashMap;

pub enum Value {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

pub struct Identifier {
    pub provider: String,
    pub name: String,
}

pub struct Entry {
    pub identifier: Identifier,
    pub value: Value,
}

pub struct Table {
    pub identifier: Identifier,
    pub entries: HashMap<String, Entry>,
}

pub struct Database {
    pub name: String,
    pub tables: HashMap<Identifier, Table>,
}