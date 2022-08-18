use std::rc::Rc;

#[derive(Debug)]
struct Truck {
    capacity: i32,
}

fn main() {
    let (truck_a, truck_b, mut truck_c) = (
        Rc::new(Truck { capacity: 3 }),
        Rc::new(Truck { capacity: 4 }),
        Rc::new(Truck { capacity: 5 }),
    );

    let facility_one = vec![truck_a, Rc::clone(&truck_b)];
    let facility_two = vec![Rc::clone(&truck_c), Rc::clone(&truck_b)];

    println!("Facility one: {:?}", facility_one);
    println!("Facility two: {:?}", facility_two);
    println!(
        "Truck B strong count: {:?}",
        std::rc::Rc::<Truck>::strong_count(&truck_b)
    );

    // Remove facility two
    std::mem::drop(facility_two);

    println!("Facility one: {:?}", facility_one);
    println!(
        "Truck B strong count: {:?}",
        std::rc::Rc::<Truck>::strong_count(&truck_b)
    );

    // Now that truck_c only has one reference, we should be able to mutate it
    Rc::get_mut(&mut truck_c).unwrap().capacity = 10;
    println!("{:?}", truck_c);
}
