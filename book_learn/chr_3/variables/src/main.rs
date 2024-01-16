// Rust’s naming convention for constants is to use all uppercase with underscores between words
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// y using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
fn main() {
    // variables and mutability
    let mut x = 5;
    println!("the value of x is:{x}!");
    x = 6;
    println!("the value of x is:{x}!");

    //constants are always immutable
    println!("3 hours in seconds is {THREE_HOURS_IN_SECONDS}");

    // shadowing
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("the value of y in the innere scope is {y}");
    }
    println!("the value of y in the outer scope is {y}");

    // with shadowing we can change the type of the value but reuse the same name.
    // this is due to we are creating a new varaible with the keyword let

    let spaces = "    ";
    let spaces = spaces.len();
    println!("_{spaces}_");

    // meanwhile, this will gives us an error
    //let mut spaces = "    ";
    //spaces = spaces.len();
}
