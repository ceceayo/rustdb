use std::collections::HashMap;

pub enum Value {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Ref(Identifier),
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

impl Entry {
    pub fn new(identifier: Identifier, value: Value) -> Entry {
        Entry {
            identifier,
            value,
        }
    }
    pub fn explain(&self) -> String {
        match &self.value {
            Value::Str(s) => format!("({}:{}): \"{}\"", self.identifier.provider, self.identifier.name, s),
            Value::Int(i) => format!("({}:{}): Int {}", self.identifier.provider, self.identifier.name, i),
            Value::Float(f) => format!("({}:{}): Float {}", self.identifier.provider, self.identifier.name, f),
            Value::Bool(b) => format!("({}:{}): {}", self.identifier.provider, self.identifier.name, b),
            Value::Ref(r) => format!("({}:{}): ({}:{})", self.identifier.provider, self.identifier.name, r.provider, r.name),
        }
    }
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
    fn sample_db_with_item() -> () {
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
        db.tables.get_mut(&Identifier {
            provider: "sample".to_string(),
            name: "planets".to_string(),
        }).unwrap().entries.insert(
            Identifier {
                provider: "unit testing app".to_string(),
                name: "mars".to_string(),
            },
            Entry {
                identifier: Identifier {
                    provider: "unit testing app".to_string(),
                    name: "mars".to_string(),
                },
                value: Value::Int(123_456_789),
            }
        );
    }
    #[test]
    fn explain_db_item() -> () {
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
        println!("{}",
        db.tables.get_mut(&Identifier {
            provider: "sample".to_string(),
            name: "planets".to_string(),
        }).unwrap().entries.get_mut(&Identifier {
            provider: "sample".to_string(),
            name: "earth".to_string(),
        }).unwrap().explain().to_string());
    }
}
