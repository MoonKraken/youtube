use std::sync::Arc;

#[derive(Debug)]
struct SharedData;

#[tokio::main]
async fn main() {
    let shared_data = Arc::new(SharedData);

    for _ in 0..3 {
        let clone_one = Arc::clone(&shared_data);
        tokio::spawn(async move {
            dbg!(&clone_one);
        });

        let clone_two = Arc::clone(&shared_data);
        tokio::spawn(async move {
            dbg!(&clone_two);
        });
    }

    // ... join all handles here
}
