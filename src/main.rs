use std::collections::HashMap;
use rustdb::{Database, Identifier, Table, Entry, Value};
fn main() {
    println!("Hello, world! JESSE");
    explain_db_item();
}


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