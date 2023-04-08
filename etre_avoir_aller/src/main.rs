use std::collections::HashMap;
use std::io;

fn main() {
    println!("Verbs Etre, Avoir and Aller");
    println!("Try to complete the correct verbs");

    let etre_verbs = HashMap::from([
        ("je", "suis"),
        ("tu", "es"),
        ("il/elle/on", "est"),
        ("nous", "sommes"),
        ("vous", "etes"),
        ("ils/elles", "sont"),
    ]);

    let avoir_verbs = HashMap::from([
        ("je", "ai"),
        ("tu", "as"),
        ("il/elle/on", "a"),
        ("nous", "avons"),
        ("vous", "avez"),
        ("ils/elles", "ont"),
    ]);

    for (a, b) in &etre_verbs {
        loop {
            println!("{a}, {b}");
            let mut verb_guess = String::new();
            io::stdin()
                .read_line(&mut verb_guess)
                .expect("Please give one of the verbs");
            let verb_guess: String = verb_guess.trim().parse().expect("Please");
            match &verb_guess == b {
                true => {
                    println!("{a} {verb_guess} est correct");
                    break;
                }
                _ => println!("essayer a nouveau!"),
            };
        }
    }

    //println!("The verb is: {verb_guess}");
    //println!("{:?}", pessoa);
}
