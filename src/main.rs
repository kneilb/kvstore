use std::collections::HashMap;

#[derive(Debug)]
enum AppErrors {
    DatabaseSaveError(std::io::Error),
    KeyNotFoundError(String),
    UnrecognisedOperationError(String)
}

fn main() -> Result<(), AppErrors> {
    let mut args = std::env::args().skip(1);
    let op = args.next().expect("No operation");
    let key = args.next().expect("No key");
    let val = if op == "set" { args.next().expect("No value") } else { String::new() };

    let mut database: Database = Database::new();

    // println!("{:?}", database);

    if op == "set" {
        println!("SET: key '{}', value '{}'", &key, &val);
        database.set_key(key, val);
    } else if op == "get" {
        match database.get_key(&key) {
            Some(val) => {
                println!("GET: key '{}', value '{}'", key, val);
            },
            None => {
                return Err(AppErrors::KeyNotFoundError(key))
            }
        }
    } else {
        return Err(AppErrors::UnrecognisedOperationError(op));
    }

    // println!("{:?}", database);

    database.save()?;

    Ok(())
}

#[derive(Debug)]
struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Self {
        let mut map: HashMap<String, String> = HashMap::new();
        match std::fs::read_to_string("kv.db") {
            Ok(contents) => {
                for line in contents.lines() {
                    let (key, val) = line.split_once('\t').expect("error!");
                    map.insert(key.to_owned(), val.to_owned());
                }
                Database {
                    map
                }
            }
            Err(_e) => {
                Database {
                    map: HashMap::new()
                }
            }
        }
    }

    fn save(&self) -> Result<(), AppErrors> {
        let mut contents: String = String::new();
        for (key, val) in &self.map {
            // contents.push_str(&format!("{}\t{}\n", key, val));
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(val);
            contents.push('\n');
        }
        
        if let Err(e) = std::fs::write("kv.db", contents) {
            Err(AppErrors::DatabaseSaveError(e))
        } else {
            Ok(())
        }
    }

    fn set_key(&mut self, key: String, val: String) {
        self.map.insert(key, val);
    }

    fn get_key(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }
}