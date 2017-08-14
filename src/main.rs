use std::env;
use std::fs::File;
use std::io::prelude::*;

fn read_the_file(word: &str) {
    let mut file = File::open("data/dict.xdxf").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let idx = contents.find(word);
    match idx {
        Some(i) => println!("Found the word at position {}", i),
        None => println!("The word is not found"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let count = env::args().len();
    match count {
        0 | 1 => {
            println!("!");
            std::process::exit(1);
        }
        2 => {
            println!("The word: {}", args[1]);
            read_the_file(&args[1]);
        }
        _ => {
            println!("{:?}", args);
            println!("Too many arguments provided: {:?}", count - 1);
            std::process::exit(2);
        }
    }
}
