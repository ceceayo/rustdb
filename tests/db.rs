use ceayo_db::{Database, Identifier, Value};

#[test]
fn test() {
    assert_eq!(2, 2);
}

#[test]
fn create_db() {
    let db = Database::new(Identifier::new("sample".to_string(), "database".to_string()));
    assert_eq!(db.tables.len(), 0);
}

#[test]
fn create_table() {
    let mut db = Database::new(Identifier::new("sample".to_string(), "database".to_string()));
    assert_eq!(db.tables.len(), 0);
    db.add(Identifier::new("sample".to_string(), "planets".to_string()));
    assert_eq!(db.tables.len(), 1);
    assert_eq!(db.tables.get_mut(&Identifier::new("sample".to_string(), "planets".to_string())).unwrap().entries.len(), 0);
}

#[test]
fn db_get_table() {
    let mut db = Database::new(Identifier::new("sample".to_string(), "database".to_string()));
    assert_eq!(db.tables.len(), 0);
    db.add(Identifier::new("sample".to_string(), "planets".to_string()));
    assert_eq!(db.tables.len(), 1);
    db.get_table(Identifier::new("sample".to_string(), "planets".to_string())).unwrap();
}

#[test]
fn create_entry() {
    let mut db = Database::new(Identifier::new("sample".to_string(), "database".to_string()));
    assert_eq!(db.tables.len(), 0);
    db.add(Identifier::new("sample".to_string(), "planets".to_string()));
    assert_eq!(db.tables.len(), 1);
    assert_eq!(db.get_table(Identifier::new("sample".to_string(), "planets".to_string())).unwrap().entries.len(), 0);
    db.get_table(Identifier::new("sample".to_string(), "planets".to_string())).unwrap()
        .add("sample".to_string(), "earth".to_string(), Value::Str("aaa".to_string()));
    assert_eq!(db.get_table(Identifier::new("sample".to_string(), "planets".to_string())).unwrap().entries.len(), 1);
}

#[test]
fn create_entries() {
    let mut db = Database::new(Identifier::new("sample".to_string(), "database".to_string()));
    assert_eq!(db.tables.len(), 0);
    db.add(Identifier::new("sample".to_string(), "planets".to_string()));
    assert_eq!(db.tables.len(), 1);
    assert_eq!(db.get_table(Identifier::new("sample".to_string(), "planets".to_string())).unwrap().entries.len(), 0);
    db.get_table(Identifier::new("sample".to_string(), "planets".to_string())).unwrap()
        .add("sample".to_string(), "earth".to_string(), Value::Str("aaa".to_string()));
    db.get_table(Identifier::new("sample".to_string(), "planets".to_string())).unwrap()
        .add("sample".to_string(), "mars".to_string(), Value::Int(123));
    db.get_table(Identifier::new("sample".to_string(), "planets".to_string())).unwrap()
        .add("sample".to_string(), "venus".to_string(), Value::Float(12.5));
    db.get_table(Identifier::new("sample".to_string(), "planets".to_string())).unwrap()
        .add("sample".to_string(), "uranus".to_string(), Value::Bool(true));
    db.get_table(Identifier::new("sample".to_string(), "planets".to_string())).unwrap()
        .add("sample".to_string(), "pluto".to_string(), Value::Id(Identifier::new("sample".to_string(), "earth".to_string())));
    db.get_table(Identifier::new("sample".to_string(), "planets".to_string())).unwrap()
        .add("testapp_2".to_string(), "pluto".to_string(), Value::LinkOut(Identifier::new("sample".to_string(), "planets".to_string()), Identifier::new("sample".to_string(), "earth".to_string())));
    db.get_table(Identifier::new("sample".to_string(), "planets".to_string())).unwrap()
        .add("testapp_2".to_string(), "mars".to_string(), Value::Table(Identifier::new("sample".to_string(), "planets".to_string())));
    
    
    assert_eq!(db.get_table(Identifier::new("sample".to_string(), "planets".to_string())).unwrap().entries.len(), 7);
}