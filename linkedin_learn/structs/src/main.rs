#[derive(Debug, Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }

    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 7,
            propellant: 0.0,
        }
    }
}

struct Color(u8, u8, u8); // RGB
struct Point(u8, u8, u8); // XYZ

fn get_y(p: Point) -> u8 {
    p.1
}

fn main() {
    let mut vehicle = Shuttle::new("Endeavour");
    let vehicle2 = Shuttle::new("Atlantis");
    let vehicle_name = vehicle.get_name();
    println!("vehicle name is {}", vehicle_name);

    vehicle.crew_size = 9;

    println!("name is {:?}", vehicle);

    println!("vehicle is {:?}", vehicle2);

    println!("propellant is {}", vehicle.propellant);
    vehicle.add_fuel(1000.0);
    println!("propellant is {}", vehicle.propellant);

    let red = Color(255, 0, 0);
    println!("First value is {}", red.0);
    let coord = Point(4, 5, 6);
    let y = get_y(coord);
    println!("y is {}", y);
}
