use crate::{Entry, Value, Table, Database};
use base64::{engine::general_purpose::STANDARD, Engine as _};

pub fn serialize_string(string: String) -> String {
    STANDARD.encode(string.as_bytes()).to_string()
}

pub fn serialize_entry(entry: Entry) -> String{
    match entry.value {
        Value::Str(s) => format!("({}:{}): Str \"{}\"", entry.identifier.provider, entry.identifier.name, s),
        Value::Int(i) => format!("({}:{}): Int {}", entry.identifier.provider, entry.identifier.name, i),
        Value::Float(f) => format!("({}:{}): Float {}", entry.identifier.provider, entry.identifier.name, f),
        Value::Bool(b) => format!("({}:{}): Bool {}", entry.identifier.provider, entry.identifier.name, b),
        Value::Id(i) => format!("({}:{}): Id ({}:{})", entry.identifier.provider, entry.identifier.name, i.provider, i.name),
        Value::LinkOut(i1, i2) => format!("({}:{}): LinkOut in table ({}:{}) -> ({}:{})", entry.identifier.provider, entry.identifier.name, i1.provider, i1.name, i2.provider, i2.name),
        Value::Table(i) => format!("({}:{}): Table ({}:{})", entry.identifier.provider, entry.identifier.name, i.provider, i.name),
    }
}

pub fn serialize_table(table: Table) -> String {
    let mut res: String = format!(">>> TABLE ({}:{})", table.identifier.provider, table.identifier.name);
    for (_, entry) in table.entries.iter() {
        res = format!("{}\n\t{}", res, serialize_entry(entry.clone()).to_string());
    }
    res.to_string()
}

pub fn serialize_database(db: Database) -> String {
    let mut res: String = format!(">> DATABASE ({}:{})\n// This is an export of a ceayo_db database.\n// https://github.com/ceceayo/rustdb\n---", db.id.provider, db.id.name);
    for (_, table) in db.tables.iter() {
        res = format!("{}\n{}", res, serialize_table(table.clone()).to_string());
    }
    res.to_string()
}