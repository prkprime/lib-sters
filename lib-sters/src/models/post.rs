use super::user;
use derive_new::new;
use std::vec::Vec;

#[derive(Debug, new, Default)]
pub struct Post {
    pub short_id: String,
    pub short_id_url: String,
    pub created_at: String,
    pub title: String,
    pub url: String,
    pub score: u32,
    pub flags: u32,
    pub comment_count: u32,
    pub description: String,
    pub comments_url: String,
    pub submitted_user: user::User,
    pub tags: Vec<String>,
}
