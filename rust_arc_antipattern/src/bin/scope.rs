#[derive(Debug)]
struct SharedData;

pub fn main() {
    let shared_data = SharedData;

    std::thread::scope(|s| {
        for _ in 0..3 {
            s.spawn(|| {
                for _ in 0..3 {
                    dbg!(&shared_data);
                }
            });

            s.spawn(|| {
                dbg!(&shared_data);
            });
        }
    });
}

