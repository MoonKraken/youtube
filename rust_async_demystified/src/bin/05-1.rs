use tokio::join;

#[tokio::main(worker_threads = 2)]
pub async fn main() {
    let a = get_page("https://news.ycombinator.com");
    let b = get_page("https://www.lobste.rs");
    join!(a, b);
}

pub async fn get_page(url: &str) {
    println!("Retrieving {}", url);
    let _ = reqwest::get(url).await.unwrap();
    println!("Completed {}", url);
}
