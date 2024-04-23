use std::collections::HashMap;

pub mod serialize;
pub mod loader;

#[derive(Clone)]
pub enum Value {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Id(Identifier),
    LinkOut(Identifier, Identifier),
    Table(Identifier),
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Identifier {
    pub provider: String,
    pub name: String,
}

impl Identifier {
    pub fn new(provider: String, name: String) -> Identifier {
        Identifier{
            provider: provider,
            name: name,
        }
    }
}

#[derive(Clone)]
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
}

#[derive(Clone)]
pub struct Table {
    pub identifier: Identifier,
    pub entries: HashMap<Identifier, Entry>,
}

impl Table {
    pub fn new(identifier: Identifier) -> Table {
        Table {
            identifier,
            entries: HashMap::new(),
        }
    }

    pub fn add(&mut self, provider: String, name: String, value: Value) -> () {
        //assert_eq!(identifier, entry.identifier);
        let _ = &self.entries.insert(Identifier{
            provider: provider.clone(),
            name: name.clone(),
        }, Entry {
            identifier: Identifier{
                provider: provider,
                name: name,
            },
            value: value,
        });
        return ();
    }

}

pub struct Database {
    pub id: Identifier,
    pub tables: HashMap<Identifier, Table>,
}

impl Database {
    pub fn new(id: Identifier) -> Database {
        Database {
            id: id,
            tables: HashMap::new(),
        }
    }
    pub fn add(&mut self, identifier: Identifier){
        let _ = &self.tables.insert(Identifier{
            provider: identifier.provider.clone(),
            name: identifier.name.clone(),
        }, Table::new(Identifier{
            provider: identifier.provider.clone(),
            name: identifier.name.clone(),
        }));
    }
    pub fn get_table(&mut self, identifier: Identifier) -> Option<&mut Table> {
        if self.tables.contains_key(&identifier.clone()) {
            Some(self.tables.get_mut(&identifier.clone()).unwrap())
        } else {
            None
        }
    }
}
