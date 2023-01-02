use std::env;
use std::fs;

fn main() {
    if env::args().len() < 2 {
        println!("Need 2 arguments, file and name");
        std::process::exit(1);
    }

    let file_path: String = env::args().nth(1).unwrap();
    let name: String = env::args().nth(2).unwrap();
    let file = fs::read_to_string(&file_path).unwrap();
    println!("Checking names");
    for line in file.lines() {
        if line.trim() == name {
            println!("The name {name} is on the list.");
            return;
        }
    }
    println!("The name {name} is not on the list.");
}
