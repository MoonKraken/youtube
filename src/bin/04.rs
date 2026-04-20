use std::{thread, thread::sleep, time::{Duration, Instant}};
pub fn main() {
    let start = Instant::now();
    let jh = thread::spawn(|| {
        sleep(Duration::new(2, 0)); // CPU bound operation
    });

    sleep(Duration::new(2, 0)); // CPU bound operation

    let _ = jh.join();
    dbg!(start.elapsed());
}
