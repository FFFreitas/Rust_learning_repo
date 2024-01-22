fn ready(y: bool) {
    if y {
        println!("y is true");
    }
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn make_and_drop() {
    let _a_box = Box::new(5);
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr. ");
    name
}

fn main() {
    // ready(x);
    let x = true;
    ready(x);

    let n = 5;
    let y = plus_one(n);
    println!("The value of y: {y}");

    let _a = 5;
    let mut _b = _a;
    _b += 1;

    // Boxes Live in the Heap
    //let _c = [0; 1_000_000];
    //let _d = _c;

    // as a box
    //let _e = Box::new([0; 1_000_000]);
    //let _f = _e;

    let _a_num = 4;
    make_and_drop();

    // Collections Use Boxes
    let first = String::from("Ferris");
    let first_clone = first.clone(); // Cloning Avoids Moves
    let full = add_suffix(first_clone);
    println!("{full}");

    // Variables Cannot Be Used After Being Moved
    println!("{full}, originaly was {first}")
}

// A foundational goal of Rust is to ensure that your programs never have undefined behavior.
