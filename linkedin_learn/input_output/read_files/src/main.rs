use std::fs;

fn main() {
    let contents = fs::read_to_string("../example.txt").unwrap();
    println!("contents in {contents}!");

    for line in contents.lines() {
        println!("line is {line}");
    }

    let contents = fs::read("../example.txt").unwrap();
    println!("contents in {contents:?}!");
}
