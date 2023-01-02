use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter a number");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("No number");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("number is too low"),
            Ordering::Greater => println!("number is too big"),
            Ordering::Equal => {
                println!("correct");
                break;
            }
        }
    }
}
