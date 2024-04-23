use crate::{Entry, Value, Table};

pub fn serialize_entry(entry: Entry) -> String{
    match entry.value {
        Value::Str(s) => format!("({}:{}): \"{}\"", entry.identifier.provider, entry.identifier.name, s),
        Value::Int(i) => format!("({}:{}): Int {}", entry.identifier.provider, entry.identifier.name, i),
        Value::Float(f) => format!("({}:{}): Float {}", entry.identifier.provider, entry.identifier.name, f),
        Value::Bool(b) => format!("({}:{}): Bool {}", entry.identifier.provider, entry.identifier.name, b),
        Value::Ref(r) => format!("({}:{}): ({}:{})", entry.identifier.provider, entry.identifier.name, r.provider, r.name),
    }
}

pub fn serialize_table(table: Table) -> String {
    let mut res: String = format!(">>> TABLE ({}:{})", table.identifier.provider, table.identifier.name);
    for (_, entry) in table.entries.iter() {
        res = format!("{}\n\t{}", res, serialize_entry(entry.clone()).to_string());
    }
    res.to_string()
}