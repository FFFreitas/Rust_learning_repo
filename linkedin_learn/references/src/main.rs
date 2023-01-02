fn main() {
    let mut rocket_fuel = String::from("RP-1");
    let lenght = process_fuel(&mut rocket_fuel);
    println!("rocket_fuel is {} and {}", rocket_fuel, lenght);
    let new_rocket_fuel = produce_fuel();
    println!("produced fuel {}", new_rocket_fuel);
}

fn process_fuel(propellant: &mut String) -> usize {
    println!("processing propelant {}", propellant);
    propellant.push_str("highly flamable");
    let lenght = propellant.len();
    lenght
}

fn produce_fuel() -> String {
    let new_fuel = String::from("RP-1");
    new_fuel //<- produces dangling references
}
