struct Sedan;
impl LandCapable for Sedan {}

struct SUV;
impl LandCapable for SUV {}

trait LandCapable {
    fn drive(&self) {
        println!("Default drive");
    }
}

trait WaterCapable {
    fn float(&self) {
        println!("Default float");
    }
}

trait Amphibious : WaterCapable + LandCapable {}

struct Hovercraft;
impl Amphibious for Hovercraft {}
impl LandCapable for Hovercraft {
    fn drive(&self) {
        println!("Hovercraft driving");
    }
}
impl WaterCapable for Hovercraft {}

fn road_trip(vehicle: &impl LandCapable) {
    vehicle.drive();
}

fn traverse_frozen_lake(vehicle: &impl Amphibious) {
    vehicle.drive();
    vehicle.float();
}

fn main() {
    let hc = Hovercraft;
    traverse_frozen_lake(&hc);
}
