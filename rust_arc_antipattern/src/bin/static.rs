use std::sync::LazyLock;

#[derive(Debug)]
struct SharedData;

static SHARED_DATA: LazyLock<SharedData> = LazyLock::new(|| {
    SharedData
});

pub fn main() {
    for _ in 0..3 {
        std::thread::spawn(|| {
            for _ in 0..3 {
                dbg!(&*SHARED_DATA);
            }
        });

        std::thread::spawn(|| {
            dbg!(&*SHARED_DATA);
        });
    }

    // ... join all handles here
}
