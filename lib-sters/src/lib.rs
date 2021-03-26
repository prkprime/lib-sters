pub mod models;

pub fn generate_url(path: String, page: Option<u32>) -> Option<String> {
    let mut url = "https://lobste.rs/".to_owned();
    if path == "newest" {
        url = format!("{}newest", url);
        match page {
            Some(page) => url = format!("{}/page/{}", url, page),
            None => {}
        }
        url = format!("{}.json", url);
        Some(url)
    } else if path == "hottest" {
        url = format!("{}hottest", url);
        url = format!("{}.json", url);
        match page {
            Some(page) => url = format!("{}?page={}", url, page),
            None => {}
        }
        Some(url)
    } else {
        None
    }
}
