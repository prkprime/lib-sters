#[cfg(test)]
mod tests {
    use lib_sters::*;
    #[test]
    fn generate_url_newest() {
        assert_eq!(
            generate_url("newest", None).unwrap(),
            "https://lobste.rs/newest.json"
        );
        assert_eq!(
            generate_url("newest", Some(654)).unwrap(),
            "https://lobste.rs/newest/page/654.json"
        );
        assert_eq!(
            generate_url("newest", Some(1u32)).unwrap(),
            "https://lobste.rs/newest/page/1.json"
        );
        assert_eq!(
            generate_url("newest", Some(599u32)).unwrap(),
            "https://lobste.rs/newest/page/599.json"
        );
    }

    #[test]
    fn generate_url_hottest() {
        assert_eq!(
            generate_url("hottest", None).unwrap(),
            "https://lobste.rs/hottest.json"
        );
        assert_eq!(
            generate_url("hottest", Some(6584)).unwrap(),
            "https://lobste.rs/hottest.json?page=6584"
        );
        assert_eq!(
            generate_url("hottest", Some(49u32)).unwrap(),
            "https://lobste.rs/hottest.json?page=49"
        );
        assert_eq!(
            generate_url("hottest", Some(3620)).unwrap(),
            "https://lobste.rs/hottest.json?page=3620"
        );
    }

    #[test]
    fn generate_url_invalid_path() {
        assert!(generate_url("invalid", None).is_none());
        assert!(generate_url("yet_another_invalid", Some(69u32)).is_none());
    }
}
