#[cfg(test)]
mod get_posts_tests {
    use lib_sters::*;

    #[test]
    fn test_get_posts_first_page() {
        let posts = get_posts(LobstersPath::Hottest, None);
        assert_ne!(posts.len(), 0)
    }

    #[test]
    fn test_get_posts_some_page() {
        let posts = get_posts(LobstersPath::Hottest, Some(69u32));
        assert_ne!(posts.len(), 0)
    }

    #[test]
    fn test_get_posts_non_existing_page() {
        let posts = get_posts(LobstersPath::Hottest, Some(999999u32));
        assert_eq!(posts.len(), 0)
    }
}

#[cfg(test)]
mod get_post_tests {
    use lib_sters::*;

    #[test]
    fn test_get_post_with_existing_id() {
        let post = get_post("sh2kcf");
        assert_eq!(post.short_id, "sh2kcf");
    }

    #[test]
    fn test_get_post_invalid_id() {
        let post = get_post("sh2kcfg");
        assert_eq!(post.short_id, "");
    }
}
