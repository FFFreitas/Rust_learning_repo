fn main() {
    let mut s1 = String::from("hello");

    let mut len = calculate_length(&s1);

    println!("The lenght of '{}' is {}.", s1, len);

    change(&mut s1);
    len = calculate_length(&s1);
    println!("The lenght of '{}' is {}.", s1, len);

    let r1 = &s1;
    let r2 = &s1;
    println!("{} {}", r1, r2);
    let r3 = &mut s1;
    println!("{}", r3);

    let s2 = &mut s1;
    consume(&mut *s2);
    println!("{}", s2);

    let mut n: i32 = 1;
    incr(&mut n);
    println!("{n}");
}

fn incr(n: &mut i32) {
    *n += 1;
}

fn consume(_s: &mut String) {}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(" world");
}
