#[derive(Debug)]
struct SharedData;

pub fn main() {
    static SHARED_DATA: &SharedData = &SharedData;

    for _ in 0..3 {
        std::thread::spawn(move || {
            dbg!(SHARED_DATA);
        });

        std::thread::spawn(move || {
            dbg!(SHARED_DATA);
        });
    }

    // ... join all handles here
}
