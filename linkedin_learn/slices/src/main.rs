fn main() {
    let message = String::from("greatings from earth");
    println!("message is {}", message);

    let last_word = &message[15..];
    println!("last word is {}", last_word);

    let planets = [1, 2, 3, 4, 5, 6, 7, 8];
    let inner_planets: &[i32] = &planets[..4];
    println!("inner planets are {:?}", inner_planets);
    let first_word = get_first_word(&message);
    println!("first word is {first_word}")
}

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }

    &s
}
