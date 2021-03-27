use derive_new::new;
use serde::Deserialize;
#[derive(Debug, new, Default, Deserialize)]
pub struct Kbsig {
    pub kb_username: String,
    pub sig_hash: String,
}
