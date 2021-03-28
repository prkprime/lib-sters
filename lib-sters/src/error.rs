use thiserror::Error;

#[derive(Debug, Error)]
pub enum LobstersError {
    #[error("failed to get post from id {0}")]
    FailedToGetPost(String),
    #[error("failed to get posts")]
    FailedToGetPosts {
        #[from]
        source: ureq::Error,
    },
    #[error("unexpected IO error")]
    IO {
        #[from]
        source: std::io::Error,
    },
    #[error("unexpected error while (de)serializing JSON response")]
    JSON {
        #[from]
        source: serde_json::Error,
    },
}
