use std::io;

fn main() {
    let _guess: u32 = "42".parse().expect("Not a number!");

    // float points
    let _x = 2.9; // f64
    let _y: f32 = 3.0; // f32

    // Numeric Operations

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _differenece = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3;

    // remainder
    let _remainder = 43 % 5;

    // Boolean
    let _t = true;
    let _f: bool = false; // explicit form

    // Character type
    let _c = 'z';
    let _z: char = 'Z';
    let _heart_eye_cat = '😻';

    // compound types
    // Tuple type
    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    // destructive pattern
    let _tup2 = (500, 6.4, 1);

    let (x, y, z) = _tup;
    println!("The value of x, y and z are: {x}, {y}, {z}");

    // We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access.

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;

    // Array type
    // Unlike a tuple, every element of an array must have the same type.
    // Unlike arrays in some other languages, arrays in Rust have a fixed length.

    let _a = [1, 2, 3, 4, 5];

    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array

    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // initialize an array
    let _a = [3; 5];

    // accessing elements in an array

    let a = [1, 2, 3, 4, 5];
    let _first = a[0];
    let _second = [1];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index passed is not a number");
    let element = a[index];
    println!("The value is {element} at the index {index}");

    let t = ([1; 2], [3, 4]);
    let (a, b) = t;
    println!("{}", a[0] + t.1[0]);
}
