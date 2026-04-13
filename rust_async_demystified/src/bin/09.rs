use tokio::join;

#[tokio::main(worker_threads = 2)]
pub async fn main() {
    let a = tokio::spawn(get_page("https://news.ycombinator.com"));
    let b = tokio::spawn(get_page("https://www.lobste.rs"));
    let c = tokio::spawn(get_page("https://www.daily.dev"));
    let d = get_page("https://www.hashnode.com");
    let _ = join!(a, b, c, d);
}

pub async fn get_page(url: &str) {
    println!("Retrieving {}", url);
    let _ = reqwest::get(url).await.unwrap();
    println!("Completed {}", url);
}
