use std::sync::LazyLock;

#[derive(Debug)]
struct SharedData;

static SHARED_DATA: LazyLock<SharedData> = LazyLock::new(|| {
    SharedData
});

#[tokio::main]
async fn main() {
    for _ in 0..3 {
        tokio::spawn(async {
            dbg!(&*SHARED_DATA);
        });

        tokio::spawn(async {
            dbg!(&*SHARED_DATA);
        });
    }

    // ... join all handles here
}
