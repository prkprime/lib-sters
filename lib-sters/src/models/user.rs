use derive_new::new;

#[derive(Debug, new)]
pub struct User {
    pub username: String,
    pub created_at: String,
    pub is_admin: bool,
    pub about: String,
    pub is_moderator: bool,
    pub karma: u32,
    pub avtar_url: String,
    pub invited_by_user: String,
    pub github_username: Option<String>,
    pub twitter_username: Option<String>,
}
