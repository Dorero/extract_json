use serde_json::{Value, from_str, to_string_pretty};
use std::{collections::HashMap, env};
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("No arguments passed. You need to pass the path to the file and keys.")
    }

    let mut file = File::open(args[1].clone()).expect("Failed to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Failed to read from file");

    let json_value: Value = from_str(&contents).expect("Filed to parsing json");

    let mut extracted: HashMap<String, Value> = HashMap::new();

    for arg in args[2..].iter() {
        let value = json_value.get(arg);
        if value.is_none() {
            panic!("Argument {} dont exist in this json", arg)
        } else {
           extracted.insert(arg.to_string(), value.unwrap().clone());
        }
    }


    let json_output = to_string_pretty(&extracted).expect("Failed to serialize json");

    println!("{}", json_output);
}
