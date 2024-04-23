use ceayo_db::{Entry, Value, Identifier};
use ceayo_db::serialize::serialize_entry;

#[test]
fn test_serialize_entry_with_string() {
    assert_eq!(serialize_entry(Entry {
        identifier: Identifier::new("sample".to_string(), "test".to_string()),
        value: Value::Str("aaa".to_string()),
    }), "(sample:test): \"aaa\"");
}

#[test]
fn test_serialize_entry_with_all_types() {
    assert_eq!(serialize_entry(Entry {
        identifier: Identifier::new("sample".to_string(), "test".to_string()),
        value: Value::Str("aaa".to_string()),
    }), "(sample:test): \"aaa\"");
    assert_eq!(serialize_entry(Entry {
        identifier: Identifier::new("sample".to_string(), "test".to_string()),
        value: Value::Int(4),
    }), "(sample:test): Int 4");
    assert_eq!(serialize_entry(Entry {
        identifier: Identifier::new("sample".to_string(), "test".to_string()),
        value: Value::Float(2.05),
    }), "(sample:test): Float 2.05");
    assert_eq!(serialize_entry(Entry {
        identifier: Identifier::new("sample".to_string(), "test".to_string()),
        value: Value::Bool(true),
    }), "(sample:test): Bool true");
    assert_eq!(serialize_entry(Entry {
        identifier: Identifier::new("sample".to_string(), "test".to_string()),
        value: Value::Ref(Identifier::new("sample".to_string(), "test".to_string())),
    }), "(sample:test): (sample:test)");
}

#[test]
fn test_serialize_entry_with_weird_floats() {
    assert_eq!(serialize_entry(Entry {
        identifier: Identifier::new("sample".to_string(), "test".to_string()),
        value: Value::Float(2.0),
    }), "(sample:test): Float 2");
    assert_eq!(serialize_entry(Entry {
        identifier: Identifier::new("sample".to_string(), "test".to_string()),
        value: Value::Float(2.5),
    }), "(sample:test): Float 2.5");
    assert_eq!(serialize_entry(Entry {
        identifier: Identifier::new("sample".to_string(), "test".to_string()),
        value: Value::Float(2.00005),
    }), "(sample:test): Float 2.00005");
    assert_eq!(serialize_entry(Entry {
        identifier: Identifier::new("sample".to_string(), "test".to_string()),
        value: Value::Float(2.0 / 5.0),
    }), "(sample:test): Float 0.4");
    
}