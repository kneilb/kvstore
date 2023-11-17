use clap::Parser;
use std::collections::HashMap;

#[derive(Debug)]
enum AppErrors {
    DatabaseSaveError(std::io::Error),
    KeyNotFoundError(String),
}

#[derive(clap::Subcommand)]
enum Operations {
    Get,
    Set {
        #[arg(short, long)]
        value: String,
    },
}

#[derive(Parser)]
#[clap(name = "kvstore", version = "0.1.0", author = "Neil Burningham")]
struct Args {
    #[command(subcommand)]
    operation: Operations,

    #[arg(short, long)]
    key: String,
}

fn main() -> Result<(), AppErrors> {
    let args = Args::parse();

    let mut database: Database = Database::new();

    // println!("{:?}", database);

    match args.operation {
        Operations::Set { value } => {
            println!("SET: key '{}' <- value '{}'", &args.key, &value);
            database.set_key(args.key, value);
        }
        Operations::Get => match database.get_key(&args.key) {
            Some(val) => {
                println!("GET: key '{}' -> value '{}'", args.key, val);
            }
            None => return Err(AppErrors::KeyNotFoundError(args.key)),
        },
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
                Database { map }
            }
            Err(_e) => Database {
                map: HashMap::new(),
            },
        }
    }

    fn save(&self) -> Result<(), AppErrors> {
        let mut contents: String = String::new();
        for (key, val) in &self.map {
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
