#[tokio::main]
pub async fn main() {
    get_page("https://news.ycombinator.com").await;
    get_page("https://www.lobste.rs").await;
}

pub async fn get_page(url: &str) {
    println!("Retrieving {}", url);
    let _ = ureq::get(url).call().unwrap();
    println!("Completed {}", url);
}
