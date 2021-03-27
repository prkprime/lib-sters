use super::user;
use derive_new::new;
use serde::Deserialize;

#[derive(Debug, new, Default, Deserialize)]
pub struct Post {
    pub short_id: String,
    pub short_id_url: String,
    pub created_at: String,
    pub title: String,
    pub url: String,
    pub score: i32,
    pub flags: u32,
    pub comment_count: Option<u32>,
    pub description: Option<String>,
    pub comments_url: String,
    pub submitter_user: user::User,
    pub tags: Option<Vec<String>>,
}
