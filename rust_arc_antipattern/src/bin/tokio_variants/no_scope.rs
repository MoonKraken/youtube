#[derive(Debug)]
struct SharedData;

#[tokio::main]
async fn main() {
    let shared_data = SharedData;

    for _ in 0..3 {
        tokio::spawn(async {
            dbg!(&shared_data);
        });

        tokio::spawn(async {
            dbg!(&shared_data);
        });
    }

    //...
}
