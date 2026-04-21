use std::{thread::sleep, time::{Duration, Instant}};

pub fn main() {
    let start = Instant::now();

    rayon::join(
        || sleep(Duration::new(2, 0)), 
        || sleep(Duration::new(2, 0))
    );

    dbg!(start.elapsed());
}
