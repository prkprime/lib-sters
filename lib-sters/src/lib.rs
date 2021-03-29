pub mod error;
pub mod models;
use error::LobstersError;
use models::Post;
use ureq;

pub enum LobstersPath {
    Newest,
    Hottest,
}

fn generate_url(path: LobstersPath, page: Option<u32>) -> String {
    format!(
        "https://lobste.rs/{}.json?page={}",
        match path {
            LobstersPath::Hottest => "hottest",
            LobstersPath::Newest => "newest",
        },
        match page {
            Some(page) => page,
            None => 1,
        }
    )
}

#[cfg(test)]
mod url_gen_tests {
    use super::{generate_url, LobstersPath};
    #[test]
    fn generate_url_newest() {
        assert_eq!(
            generate_url(LobstersPath::Newest, None),
            "https://lobste.rs/newest.json?page=1"
        );
        assert_eq!(
            generate_url(LobstersPath::Newest, Some(654)),
            "https://lobste.rs/newest.json?page=654"
        );
        assert_eq!(
            generate_url(LobstersPath::Newest, Some(1u32)),
            "https://lobste.rs/newest.json?page=1"
        );
        assert_eq!(
            generate_url(LobstersPath::Newest, Some(599u32)),
            "https://lobste.rs/newest.json?page=599"
        );
    }

    #[test]
    fn generate_url_hottest() {
        assert_eq!(
            generate_url(LobstersPath::Hottest, None),
            "https://lobste.rs/hottest.json?page=1"
        );
        assert_eq!(
            generate_url(LobstersPath::Hottest, Some(6584)),
            "https://lobste.rs/hottest.json?page=6584"
        );
        assert_eq!(
            generate_url(LobstersPath::Hottest, Some(49u32)),
            "https://lobste.rs/hottest.json?page=49"
        );
        assert_eq!(
            generate_url(LobstersPath::Hottest, Some(3620)),
            "https://lobste.rs/hottest.json?page=3620"
        );
    }
}

pub fn get_posts(path: LobstersPath, page: Option<u32>) -> Result<Vec<Post>, LobstersError> {
    let url = generate_url(path, page);
    let r = ureq::get(&url);
    return match r.call() {
        Ok(response) => Ok(response.into_json::<Vec<Post>>()?),
        Err(err) => Err(LobstersError::FailedToGetPosts { source: err }),
    };
}

pub fn get_post(post_id: &str) -> Result<Post, LobstersError> {
    let url: String = format!("https://lobste.rs/s/{}.json", post_id);
    let r = ureq::get(&url);
    return match r.call() {
        Ok(response) => Ok(response.into_json::<Post>()?),
        Err(err) => Err(LobstersError::FailedToGetPosts { source: err }),
    };
}
