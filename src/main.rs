use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("Key was not there");
    let value = arguments.next().unwrap();
    println!("The key is {}", key);
    println!("The value is {}", value);

    // let contents = format!("{}\t{}\n", key, value);
    // std::fs::write("kv.db", contents).unwrap();
    let mut db = Database::new().expect("Database::new crashed");
    // Same as: Database::insert(db, key, value)
    db.insert(key.to_uppercase(), value.clone());
    db.insert(key, value);
    match db.flush() {
        Ok(()) => println!("Yay!"),
        Err(err) => println!("Oh No! Error {}", err),
    }
    // Can't do that because flush has taken ownership
    // db.insert(String::from("Wow"), String::from("WOO"))
}

struct Database {
    map: HashMap<String, String>,
    flush: bool,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // read the kv.db file
        // let content = match std::fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(err) => {
        //         return Result::Err(err);
        //     }
        // };
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            // parse the string
            let (key, value) = line.split_once('\t').expect("Corrupt database.");
            // populate our map
            map.insert(key.to_owned(), value.to_owned());
        }

        Result::Ok(Database { map, flush: false })
    }

    fn insert(&mut self, key: String, value: String) -> () {
        self.map.insert(key, value);
    }

    fn flush(mut self) -> std::io::Result<()> {
        self.flush = true;
        do_flush(&self)
    }
}

// Called automatically when a value is dropped.
impl Drop for Database {
    fn drop(&mut self) {
        if !self.flush {
            let _ = do_flush(self);
        }
    }
}

fn do_flush(db: &Database) -> std::io::Result<()> {
    println!("Do flush called...");
    let mut contents = String::new();
    for (key, value) in &db.map {
        // let kvpair = format!("{}\t{}\n", key, value);
        // contents.push_str(&kvpair)
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');
    }
    std::fs::write("kv.db", contents)
}
