// Traits are like Java Interfaces
pub trait Vehicle {
    fn forward(&self) -> String;
    fn backward(&self) -> String;
    fn turn_ignition() -> String {
        String::from("vroom vroom")
    }
}

pub struct Car {
    color: String
}
impl Vehicle for Car {
    fn forward(&self) -> String {
        String::from(format!("Driving my {} Car Forward", self.color))
    }
    fn backward(&self) -> String {
        String::from("Driving car Backward")
    }
}

pub struct Truck {
    color: String
}
impl Vehicle for Truck {
    fn forward(&self) -> String {
        String::from(format!("Driving my {} Truck Forward", self.color))
    }
    fn backward(&self) -> String {
        String::from("Driving Truck Backward")
    }
}

pub fn traits_example() {
    let car_1 = Car{ color: String::from("blue") };
    println!("{}", Car::turn_ignition());
    println!("{}", car_1.forward());
    println!("{}", car_1.backward());

    let truck_1 = Truck{ color: String::from("green") };
    println!("{}", Truck::turn_ignition());
    println!("{}", truck_1.forward());
    println!("{}", truck_1.backward());
}