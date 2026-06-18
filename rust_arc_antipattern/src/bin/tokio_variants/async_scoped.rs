use async_scoped::TokioScope;

#[derive(Debug)]
struct SharedData;

#[tokio::main]
async fn main() {
    let shared_data = SharedData;

    let ((), _) = TokioScope::scope_and_block(|s| {
        for _ in 0..3 {
            s.spawn(async {
                dbg!(&shared_data);
            });

            s.spawn(async {
                dbg!(&shared_data);
            });
        }
    });
}
