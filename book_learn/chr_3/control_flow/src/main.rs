fn main() {
    let number = 3;
    if number < 5 {
        println!(" condition was true");
    } else {
        println!("condition was false");
    };

    if number != 0 {
        println!(" number was something else other than 0")
    }

    // Multiple conditions

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 2, 3 or 4");
    };

    let condition = true;
    let new_number = if condition { 5 } else { 6 };
    println!("The value of a new number is {new_number}");
    // let _another_number = if condition { 5 } else { "six" };
    // loops

    loop {
        println!("again");
        break;
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result from counting: {result}");

    // labeled loops

    let mut count = 0;
    'counting_up: loop {
        println!("counting = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("Blastoff!!!");

    // Looping Through a Collection with for

    let a = [10, 20, 30, 40, 50, 60];
    let mut index = 0;

    while index < 5 {
        println!("value is: {}", a[index]);
        index += 1;
    }
    // better way
    for element in a {
        println!("value is: {element}");
    }
}
