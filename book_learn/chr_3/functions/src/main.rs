fn main() {
    println!("Hello, world!");

    let _y = 6;

    another_function();
    another_function_par(5);
    print_labeled_measurements(5, 'h');

    let _y2 = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {_y2}");

    let x2 = five();
    println!("The value of x is: {x2}");

    let x3 = plus_one(5);
    println!("The value of x is: {x3}");

    println!(
        "{}",
        plus_one({
            let y = 1;
            y + 1
        })
    );
}

fn five() -> i32 {
    5
}

fn another_function() {
    println!("Another function")
}

fn another_function_par(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
