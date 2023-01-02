fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("{y}");

    let x = five();

    println!("{x}");

    let x = plus_one(x);

    println!("{x}");

    another_function(50);
    print_labeled_measurement(5, 'h');

    println!(
        "{}",
        f({
            let y = 1;
            y + 1
        })
    );
}

fn f(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32) {
    println!("The value of x is :{x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is : {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
