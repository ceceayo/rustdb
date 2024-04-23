use std::collections::HashMap;

pub enum Value {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Eq, Hash, PartialEq)]
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
    pub entries: HashMap<Identifier, Entry>,
}

pub struct Database {
    pub name: String,
    pub tables: HashMap<Identifier, Table>,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn sample_db() -> () {
        let mut db = Database {
            name: "sample".to_string(),
            tables: HashMap::new(),
        };
        db.tables.insert(
            Identifier {
                provider: "sample".to_string(),
                name: "planets".to_string(),
            },
            Table {
                identifier: Identifier {
                    provider: "sample".to_string(),
                    name: "planets".to_string(),
                },
                entries: HashMap::new(),
            }
        );
    }

    #[test]
    fn sample_db_with_items() -> () {
        let mut db = Database {
            name: "sample".to_string(),
            tables: HashMap::new(),
        };
        db.tables.insert(
            Identifier {
                provider: "sample".to_string(),
                name: "planets".to_string(),
            },
            Table {
                identifier: Identifier {
                    provider: "sample".to_string(),
                    name: "planets".to_string(),
                },
                entries: HashMap::new(),
            }
        );
        db.tables.get_mut(&Identifier {
            provider: "sample".to_string(),
            name: "planets".to_string(),
        }).unwrap().entries.insert(
            Identifier {
                provider: "sample".to_string(),
                name: "earth".to_string(),
            },
            Entry {
                identifier: Identifier {
                    provider: "sample".to_string(),
                    name: "earth".to_string(),
                },
                value: Value::Str("blue".to_string()),
            }
        );
    }
}
