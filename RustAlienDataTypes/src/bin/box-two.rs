#[derive(Debug)]
struct Truck {
    next_truck: Option<Box<Truck>>,
}

fn main() {
    println!("{:?}",Truck { next_truck: None });
}
