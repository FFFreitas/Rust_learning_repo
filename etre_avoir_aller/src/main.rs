use std::collections::HashMap;
use std::io;
use std::iter;

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

    let aller_verbs = HashMap::from([
        ("je", "vais"),
        ("tu", "vas"),
        ("il/elle/on", "va"),
        ("nous", "allons"),
        ("vous", "allez"),
        ("ils/elles", "vont"),
    ]);

    let repeated: String = iter::repeat("-*-").take(20).collect();
    for (a, b) in &etre_verbs {
        loop {
            println!("complet pour le verbe etre");
            println!("{a} ...");
            let mut verb_guess = String::new();
            io::stdin()
                .read_line(&mut verb_guess)
                .expect("Veuillez fournir un verbe");
            let verb_guess: String = match verb_guess.trim().parse() {
                Ok(str) => str,
                Err(_) => continue,
            };
            match &verb_guess == b {
                true => {
                    println!("{a} {verb_guess} est correct");
                    break;
                }
                _ => println!("essayer a nouveau!"),
            };
        }
    }

    println!("{:?}", &repeated);
    for (a, b) in &avoir_verbs {
        loop {
            println!("complet pour le verbes avoir");
            println!("{a} ...");
            let mut verb_guess = String::new();
            io::stdin()
                .read_line(&mut verb_guess)
                .expect("Veuillez fournir un verbe");
            let verb_guess: String = match verb_guess.trim().parse() {
                Ok(str) => str,
                Err(_) => continue,
            };
            match &verb_guess == b {
                true => {
                    println!("{a} {verb_guess} est correct");
                    break;
                }
                _ => println!("essayer a nouveau!"),
            };
        }
    }

    println!("{:?}", &repeated);
    for (a, b) in &aller_verbs {
        loop {
            println!("complet pour le verbes aller");
            println!("{a} ...");
            let mut verb_guess = String::new();
            io::stdin()
                .read_line(&mut verb_guess)
                .expect("Veuillez fournir un verbe");
            let verb_guess: String = match verb_guess.trim().parse() {
                Ok(str) => str,
                Err(_) => continue,
            };
            match &verb_guess == b {
                true => {
                    println!("{a} {verb_guess} est correct");
                    break;
                }
                _ => println!("essayer a nouveau!"),
            };
        }
    }
    println!("félicitations, tu as bien fait !")
}
