fn main() {
    let s1 = gives_ownership();

    let s = String::from("hello");

    let s3 = takes_and_gives_back(s);
    // takes_ownership(s);
    // s.push_str("aaa");

    let x = 5;

    makes_copy(x);
    let y: i32 = 1;
    println!("{}", y + x);

    let s4;
    let b = false;
    if b {
        s4 = s;
    }
    println!("{s}");
    let sa = String::from("hello");
    add_sufix(sa);
    println!("{sa}")
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn add_sufix(mut s: String) {
    s.push_str(" world");
}
