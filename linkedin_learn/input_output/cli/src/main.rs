use std::env;

fn main() {
    if env::args().len() <= 2 {
        println!("not enought args");
        return;
    }
    for (index, argument) in env::args().enumerate() {
        println!("arguments are {} {}", index, argument);
    }
    let arg2 = env::args().nth(2).unwrap();
    println!("arg 2 is {arg2}");
}
