use crate::{Entry, Value};

pub fn serialize_entry(entry: Entry) -> String{
    match entry.value {
        Value::Str(s) => format!("({}:{}): \"{}\"", entry.identifier.provider, entry.identifier.name, s),
        Value::Int(i) => format!("({}:{}): Int {}", entry.identifier.provider, entry.identifier.name, i),
        Value::Float(f) => format!("({}:{}): Float {}", entry.identifier.provider, entry.identifier.name, f),
        Value::Bool(b) => format!("({}:{}): Bool {}", entry.identifier.provider, entry.identifier.name, b),
        Value::Ref(r) => format!("({}:{}): ({}:{})", entry.identifier.provider, entry.identifier.name, r.provider, r.name),
    }
}