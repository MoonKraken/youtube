trait Vehicle {
    fn drive(&self);
}

struct Truck;

impl Vehicle for Truck {
    fn drive(&self) {
        println!("Truck driving");
    }
}

fn main() {
    let t: Box<dyn Vehicle>;
    t = Box::new(Truck);
    t.drive();
}
