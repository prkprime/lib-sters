#[cfg(test)]
mod tests {
    use lib_sters::*;
    #[test]
    fn generate_url_newest() {
        assert_eq!(
            generate_url("newest".to_owned(), None).unwrap(),
            "https://lobste.rs/newest.json"
        );
        assert_eq!(
            generate_url("newest".to_owned(), Some(654)).unwrap(),
            "https://lobste.rs/newest/page/654.json"
        );
        assert_eq!(
            generate_url("newest".to_owned(), Some(1u32)).unwrap(),
            "https://lobste.rs/newest/page/1.json"
        );
        assert_eq!(
            generate_url("newest".to_owned(), Some(599u32)).unwrap(),
            "https://lobste.rs/newest/page/599.json"
        );
    }

    #[test]
    fn generate_url_hottest() {
        assert_eq!(
            generate_url("hottest".to_owned(), None).unwrap(),
            "https://lobste.rs/hottest.json"
        );
        assert_eq!(
            generate_url("hottest".to_owned(), Some(6584)).unwrap(),
            "https://lobste.rs/hottest.json?page=6584"
        );
        assert_eq!(
            generate_url("hottest".to_owned(), Some(49u32)).unwrap(),
            "https://lobste.rs/hottest.json?page=49"
        );
        assert_eq!(
            generate_url("hottest".to_owned(), Some(3620)).unwrap(),
            "https://lobste.rs/hottest.json?page=3620"
        );
    }

    #[test]
    fn generate_url_invalid_path() {
        assert!(generate_url("invalid".to_owned(), None).is_none());
        assert!(generate_url("yet_another_invalid".to_owned(), Some(69u32)).is_none());
    }
}
