use async_scoped::TokioScope;

#[derive(Debug)]
struct SharedData;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let shared_data = SharedData;

    for _ in 0..3 {
        let ((), _) = TokioScope::scope_and_block(|s| {
            s.spawn(async {
                for _ in 0..3 {
                    dbg!(&shared_data);
                }
            });

            s.spawn(async {
                dbg!(&shared_data);
            });
        });
    }
}
