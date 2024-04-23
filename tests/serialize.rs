use ceayo_db::{Entry, Value, Identifier, Table};
use ceayo_db::serialize::{serialize_entry, serialize_table, serialize_string};

#[test]
fn test_serialize_entry_with_string() {
    assert_eq!(serialize_entry(Entry {
        identifier: Identifier::new("sample".to_string(), "test".to_string()),
        value: Value::Str("aaa".to_string()),
    }), "(sample:test): Str \"aaa\"");
}

#[test]
fn test_serialize_entry_with_all_types() {
    assert_eq!(serialize_entry(Entry {
        identifier: Identifier::new("sample".to_string(), "test".to_string()),
        value: Value::Str("aaa".to_string()),
    }), "(sample:test): Str \"aaa\"");
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
        value: Value::Id(Identifier::new("sample".to_string(), "test".to_string())),
    }), "(sample:test): Id (sample:test)");
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

#[test]
fn test_serialize_table() {
    let mut table = Table::new(Identifier::new("sample".to_string(), "planets".to_string()));
    table.add("sample".to_string(), "earth".to_string(), Value::Str("blue and green".to_string()));
    assert_eq!(serialize_table(table), ">>> TABLE (sample:planets)\n\
        \t(sample:earth): Str \"blue and green\"");
}

#[test]
fn test_serialize_table_with_multiple_entries() {
    let mut table = Table::new(Identifier::new("sample".to_string(), "planets".to_string()));
    table.add("sample".to_string(), "earth".to_string(), Value::Str("blue and green".to_string()));
    table.add("sample".to_string(), "mars".to_string(), Value::Str("red".to_string()));
    table.add("sample".to_string(), "jupiter".to_string(), Value::Str("big".to_string()));
    

    // I'm so so sorry
    assert!([">>> TABLE (sample:planets)\n\
        \t(sample:earth): Str \"blue and green\"\n\
        \t(sample:mars): Str \"red\"\n\
        \t(sample:jupiter): Str \"big\"".to_string(),
        ">>> TABLE (sample:planets)\n\
        \t(sample:earth): Str \"blue and green\"\n\
        \t(sample:jupiter): Str \"big\"\n\
        \t(sample:mars): Str \"red\"".to_string(),
        ">>> TABLE (sample:planets)\n\
        \t(sample:mars): Str \"red\"\n\
        \t(sample:earth): Str \"blue and green\"\n\
        \t(sample:jupiter): Str \"big\"".to_string(),
        ">>> TABLE (sample:planets)\n\
        \t(sample:jupiter): Str \"big\"\n\
        \t(sample:mars): Str \"red\"\n\
        \t(sample:earth): Str \"blue and green\"".to_string(),
        ">>> TABLE (sample:planets)\n\
        \t(sample:mars): Str \"red\"\n\
        \t(sample:jupiter): Str \"big\"\n\
        \t(sample:earth): Str \"blue and green\"".to_string(),
        ">>> TABLE (sample:planets)\n\
        \t(sample:jupiter): Str \"big\"\n\
        \t(sample:earth): Str \"blue and green\"\n\
        \t(sample:mars): Str \"red\"".to_string()].contains(&serialize_table(table)));
}

#[test]
fn test_serialize_string() {
    assert_eq!(serialize_string("hello".to_string()), "aGVsbG8=");
    assert_eq!(serialize_string("test".to_string()), "dGVzdA==");
    assert_eq!(serialize_string("abc123!!,.?=+".to_string()), "YWJjMTIzISEsLj89Kw==");
    assert_eq!(serialize_string("💕 copying is an act of love 💕".to_string()), "8J+SlSBjb3B5aW5nIGlzIGFuIGFjdCBvZiBsb3ZlIPCfkpU=");
}