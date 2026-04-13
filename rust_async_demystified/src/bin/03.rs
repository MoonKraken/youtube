pub fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        get_page("https://news.ycombinator.com").await;
        get_page("https://www.lobste.rs").await;
    });
}

pub async fn get_page(url: &str) {
    println!("Retrieving {}", url);
    let _ = ureq::get(url).call().unwrap();
    println!("Completed {}", url);
}
