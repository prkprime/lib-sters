use super::kbsig::Kbsig;
use derive_new::new;
use serde::Deserialize;
#[derive(Debug, new, Default, Deserialize)]
pub struct User {
    pub username: String,
    pub created_at: String,
    pub is_admin: bool,
    pub about: Option<String>,
    pub is_moderator: bool,
    pub karma: Option<i32>,
    pub avtar_url: Option<String>,
    pub invited_by_user: String,
    pub github_username: Option<String>,
    pub twitter_username: Option<String>,
    pub keybase_signatures: Option<Vec<Kbsig>>,
}
