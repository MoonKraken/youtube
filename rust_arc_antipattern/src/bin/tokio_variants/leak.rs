#[derive(Debug)]
struct SharedData;

#[tokio::main]
async fn main() {
    let shared_data = Box::new(SharedData);
    let shared_data: &'static SharedData = Box::leak(shared_data);

    for _ in 0..3 {
        tokio::spawn(async move {
            dbg!(shared_data);
        });

        tokio::spawn(async move {
            dbg!(shared_data);
        });
    }

    // ... join all handles here
}
