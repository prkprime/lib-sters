use serde::Deserialize;
#[derive(Debug, Default, Deserialize)]
pub struct Kbsig {
    pub kb_username: String,
    pub sig_hash: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct User {
    pub username: String,
    pub created_at: String,
    pub is_admin: bool,
    pub about: Option<String>,
    pub is_moderator: bool,
    pub karma: Option<i32>,
    pub avatar_url: Option<String>,
    pub invited_by_user: String,
    pub github_username: Option<String>,
    pub twitter_username: Option<String>,
    pub keybase_signatures: Option<Vec<Kbsig>>,
}

#[derive(Debug, Default, Deserialize)]
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
    pub submitter_user: User,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Comment {
    pub short_id: String,
    pub short_id_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub is_deleted: bool,
    pub is_moderated: bool,
    pub score: i32,
    pub flags: u32,
    pub comment: String,
    pub url: String,
    pub indent_level: i32,
    pub commenting_user: User,
}

#[derive(Debug, Default, Deserialize)]
pub struct PostDetails {
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
    pub submitter_user: User,
    pub tags: Option<Vec<String>>,
    pub comments: Option<Vec<Comment>>,
}
