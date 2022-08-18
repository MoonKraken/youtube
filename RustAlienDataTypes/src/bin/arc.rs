use std::sync::Arc;

#[derive(Debug)]
struct Truck {
    capacity: i32,
}

fn main() {
    let (truck_a, truck_b, truck_c) = (
        Arc::new(Truck { capacity: 3 }),
        Arc::new(Truck { capacity: 4 }),
        Arc::new(Truck { capacity: 5 }),
    );

    let thread = std::thread::spawn(move || {
        let facility_one = vec![truck_a, Arc::clone(&truck_b)];
        let facility_two = vec![truck_c, Arc::clone(&truck_b)];
        (facility_one, facility_two)
    });

    let (facility_one, facility_two) = thread.join().unwrap();

    //get truck_b
    let truck_b = Arc::clone(&facility_one[1]);
    println!("Facility one: {:?}", facility_one);
    println!("Facility two: {:?}", facility_two);
    println!(
        "Truck B strong count: {:?}",
        Arc::<Truck>::strong_count(&truck_b)
    );

    // Remove facility two
    std::mem::drop(facility_two);

    println!("Facility one: {:?}", facility_one);
    println!(
        "Truck B strong count: {:?}",
        Arc::<Truck>::strong_count(&truck_b)
    );
}
