use std::env;

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
        }
        _ => {
            println!("{:?}", args);
            println!("Too many arguments provided: {:?}", count - 1);
            std::process::exit(2);
        }
    }
}
