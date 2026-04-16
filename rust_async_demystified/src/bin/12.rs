use futures::future::join_all;

#[tokio::main(worker_threads = 2)]
pub async fn main() {
    let handles = (0..=500).map(|_|  {
        tokio::task::spawn_blocking(|| {
            get_page("https://news.ycombinator.com")
        })
    });

    join_all(handles).await;
}

fn get_page(url: &str) {
    println!("Retrieving {}", url);
    let _ = ureq::get(url).call().unwrap();
    println!("Completed {}", url);
}
