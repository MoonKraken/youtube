#[derive(Debug)]
struct SharedData;

pub fn main() {
    let shared_data = SharedData {};

    for _ in 0..3 {
        std::thread::spawn(|| {
            for _ in 0..3 {
                dbg!(&shared_data);
            }
        });

        std::thread::spawn(|| {
            dbg!(&shared_data);
        });
    }

    //...
}
