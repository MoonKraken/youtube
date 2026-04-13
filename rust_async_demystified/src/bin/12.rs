use futures::future::join_all;

#[tokio::main(worker_threads = 2)]
pub async fn main() {
    let handles = (0..5).map(|_|  {
        tokio::spawn(get_page("https://news.ycombinator.com"))
    });

    join_all(handles).await;
}

pub async fn get_page(url: &str) {
    println!("Retrieving {}", url);
    let _ = ureq::get(url).call().unwrap();
    println!("Completed {}", url);
}
