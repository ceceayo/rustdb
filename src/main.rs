use std::collections::HashMap;
use rustdb::*;
fn main() {
    println!("Hello, world! JESSE");
    add_function();
}

fn add_function() -> () {
    let mut db = Database {
        name: "sample".to_string(),
        tables: HashMap::new(),
    };
    db.tables.insert(Identifier{
        provider: "sample".to_string(),
        name: "planets".to_string(),
    },Table::new(Identifier{
        provider: "sample".to_string(),
        name: "planets".to_string(),
    }));
    db.tables.get_mut(&Identifier {
        provider: "sample".to_string(),
        name: "planets".to_string(),
    }).unwrap().add("unittests".to_string(), "mars".to_string(), Value::Str("hi".to_string()));
    println!("{}", db.tables.get_mut(&Identifier {
        provider: "sample".to_string(),
        name: "planets".to_string(),
    }).unwrap().entries.get_mut(&Identifier{
        provider: "unittests".to_string(),
        name: "mars".to_string(),
    }).unwrap().explain().to_string());
}