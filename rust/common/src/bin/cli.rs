use std::env;
use std::fs;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        // read file
        let path = &args[1];
        match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to read {}: {}", path, e);
                std::process::exit(2);
            }
        }
    } else {
        // read stdin
        let mut s = String::new();
        if io::stdin().read_to_string(&mut s).is_err() {
            eprintln!("Failed to read stdin");
            std::process::exit(2);
        }
        s
    };

    match markmap_common::process_tree_json(&input) {
        Ok(out) => println!("{}", out),
        Err(e) => {
            eprintln!("Processing error: {}", e);
            std::process::exit(3);
        }
    }
}
