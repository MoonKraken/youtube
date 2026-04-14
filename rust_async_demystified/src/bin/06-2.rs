use tokio::join;

#[tokio::main(worker_threads = 2)]
pub async fn main() {
    let a = tokio::spawn(get_page("https://news.ycombinator.com"));
    let b = tokio::spawn(get_page("https://www.lobste.rs"));
    let _ = join!(a, b);
}

pub async fn get_page(url: &str) {
    println!("Retrieving {}", url);
    let _ = ureq::get(url).call().unwrap();
    println!("Completed {}", url);
}
