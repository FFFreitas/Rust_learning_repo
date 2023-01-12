use std::fs;

fn main() {
    let contents = fs::read_to_string("the_ultimate_question.txt").expect("nobody knows it");
    println!("contents is: {:?}", contents);
}
