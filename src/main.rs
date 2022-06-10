use std::collections::HashMap;

#[derive(Debug)]
enum AppErrors {
    SaveError(std::io::Error),
}

fn main() -> Result<(), AppErrors> {
    let mut args = std::env::args().skip(1);
    let key = args.next().unwrap();
    let val = args.next().unwrap();

    println!("{} <- {}", key, val);

    let mut database: Database = Database::new();

    println!("{:?}", database);

    database.add_key(key, val);

    println!("{:?}", database);

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
                    map: map
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
        for (key, val) in self.map.iter() {
            contents.push_str(&format!("{}\t{}\n", key, val));
        }
        
        std::fs::write("kv.db", contents).expect("Failed to write database");
        Ok(())
    }

    fn add_key(&mut self, key: String, val: String) {
        self.map.insert(key, val);
    }
}