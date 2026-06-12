#[derive(Debug)]
struct SharedData;

pub fn main() {
    let shared_data = Box::new(SharedData);
    let shared_data: &'static SharedData = Box::leak(shared_data);

    for _ in 0..3 {
        std::thread::spawn(move || {
            for _ in 0..3 {
                dbg!(shared_data);
            }
        });

        std::thread::spawn(move || {
            dbg!(shared_data);
        });
    }

    // ... join all handles here
}
