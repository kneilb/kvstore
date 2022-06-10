use std::collections::HashMap;

fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().unwrap();
    let val = args.next().unwrap();

    let contents = format!("{}\t{}\n", key, val);

    std::fs::write("kv.db", contents).expect("Failed to write database");

    println!("{} <- {}", key, val);

    let database: Database = Database::new().unwrap();

    println!("{:?}", database);
}

#[derive(Debug)]
struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Self, std::io::Error> {
        let mut map: HashMap<String, String> = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let (key, val) = line.split_once('\t').expect("error!");
            map.insert(key.to_owned(), val.to_owned());
        }
        Ok(Database {
            map: map
        })
    }
}