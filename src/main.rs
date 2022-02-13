use std::fs;
use std::collections::HashMap;

struct Database {
    kvl_map: HashMap<String, String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // Standard Pattern matching
        // let data = match fs::read_to_string("credman.db") {
        //     Ok(contents) => {
        //         contents;
        //     }
        //     Err(error) => {
        //         return Err(error);
        //     }
        // };

        // Shorthand / common pattern matching
        
        Database {
            kvl_map: HashMap::new(),
        }
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let lookup = args.next().unwrap();
    let username = args.next().unwrap();
    let password = args.next().unwrap();

    println!("User: {} with Password: {} stored as \"{}\"", username, password, lookup);

    let contents = format!("{}\t{}\t{}\n", lookup, username, password);
    let write_result = fs::write("credman.db", contents).expect("Failed to write to disk, do you have permission?");

}