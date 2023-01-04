fn main() {
    let my_number = 1u8;

    let result = match my_number {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        _ => {
            println!("The number {} did not match", my_number);
            "something else"
        }

    };
    println!("result is {}", result)
}
