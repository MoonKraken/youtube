#[tokio::main(worker_threads = 2)]
pub async fn main() {
    get_page("https://news.ycombinator.com").await;
    get_page("https://www.lobste.rs").await;
}

pub async fn get_page(url: &str) {
    println!("Retrieving {}", url);
    let _ = reqwest::get(url).await.unwrap();
    println!("Completed {}", url);
}
