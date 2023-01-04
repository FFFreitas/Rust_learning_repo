fn main() {
    let message = String::from("Greetings from Earth!");
    let first_word = get_first_word(&message);
    println!("first_word is {}", first_word);
}

fn get_first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; // found a space!
        }
    }

    &s // no spaces found; input is a single word
}
// each input parameter that is a referece is assigned its own lifetime]// if is exactly one input
// lifetime, assign it ti all output lifetimes
// if there is a &sekf ir &mut self input parameter, its lifetime will be assigned to all output
// lifetimes
//
