use std::sync::Arc;

#[derive(Debug)]
struct SharedData;

pub fn main() {
    let shared_data = Arc::new(SharedData);

    for _ in 0..3 {
        let clone_one = Arc::clone(&shared_data);
        std::thread::spawn(move || {
            for _ in 0..3 {
                dbg!(&clone_one);
            }
        });

        let clone_two = Arc::clone(&shared_data);
        std::thread::spawn(move || {
            dbg!(&clone_two);
        });
    }

    // ... join all handles here
}
