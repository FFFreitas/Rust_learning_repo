fn main() {
    let my_string = String::from("Hello world");

    let word = first_world_new(&my_string[0..6]);
    println!("first word is {}", word);
    let word = first_world_new(&my_string[..]);
    println!("first word is {}", word);

    let my_string_literal = "hello world";

    let word = first_world_new(&my_string_literal[..6]);
    println!("first word is {}", word);
    let word = first_world_new(&my_string_literal[..]);
    println!("first word is {}", word);
    let word = first_world_new(&my_string_literal);
    println!("first word is {}", word);
}

fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_world_new(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
