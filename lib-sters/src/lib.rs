pub mod models;
use minreq::{self, Response};
use models::{post::Post, user::User};
use serde_json::Value;

pub enum LobstersPath {
    Newest,
    Hottest,
}

fn generate_url(path: LobstersPath, page: Option<u32>) -> String {
    let mut url: String = "https://lobste.rs/".to_owned();
    match path {
        LobstersPath::Newest => {
            url = format!("{}newest", url);
            match page {
                Some(page) => url = format!("{}/page/{}", url, page),
                None => {}
            }
            url = format!("{}.json", url);
            url
        }
        LobstersPath::Hottest => {
            url = format!("{}hottest", url);
            url = format!("{}.json", url);
            match page {
                Some(page) => url = format!("{}?page={}", url, page),
                None => {}
            }
            url
        }
    }
}

pub fn get_posts(path: LobstersPath, page: Option<u32>) -> Option<Vec<Post>> {
    let url = generate_url(path, page);
    let response: Response = minreq::get(url).send().unwrap();
    let res_str: &str = response.as_str().unwrap();
    let json_value: Value = serde_json::from_str(res_str).unwrap();
    let obj_vec: &Vec<Value> = json_value.as_array().unwrap();
    let mut posts: Vec<Post> = Vec::new();
    for post_obj in obj_vec {
        let post = parse_post(post_obj);
        posts.push(post);
    }
    Some(posts)
}

#[test]
fn test_get_posts() {
    let posts = get_posts(LobstersPath::Hottest, None).unwrap();
    println!("{:?}", posts[0]);
    assert_ne!(posts.len(), 0)
}

pub fn get_post(post_id: &str) -> Post {
    let url: String = format!("https://lobste.rs/s/{}.json", post_id);
    let response: Response = minreq::get(url).send().unwrap();
    let res_str: &str = response.as_str().unwrap();
    let json_value: Value = serde_json::from_str(res_str).unwrap();
    let post = parse_post(&json_value);
    post
}

#[test]
fn test_get_post() {
    let post = get_post("sh2kcf");
    assert_eq!(post.short_id, "sh2kcf");
}

fn parse_post(post_obj: &Value) -> Post {
    let mut post: Post = Post::default();
    let mut tags: Vec<String> = Vec::new();
    let user_obj = post_obj.get("submitter_user").unwrap();
    post.short_id = post_obj
        .get("short_id")
        .unwrap()
        .as_str()
        .unwrap()
        .to_owned();
    post.short_id_url = post_obj
        .get("short_id_url")
        .unwrap()
        .as_str()
        .unwrap()
        .to_owned();
    post.created_at = post_obj
        .get("created_at")
        .unwrap()
        .as_str()
        .unwrap()
        .to_owned();
    post.title = post_obj.get("title").unwrap().as_str().unwrap().to_owned();
    post.url = post_obj.get("url").unwrap().as_str().unwrap().to_owned();
    post.score = post_obj.get("score").unwrap().as_i64().unwrap() as i32;
    post.flags = post_obj.get("flags").unwrap().as_i64().unwrap() as u32;
    match post_obj.get("comment_count") {
        Some(comment_count) => {
            post.comment_count = Some(comment_count.as_i64().unwrap() as u32);
        }
        None => {}
    };
    match post_obj.get("description") {
        Some(description) => {
            post.description = Some(description.as_str().unwrap().to_owned());
        }
        None => {}
    };
    post.comments_url = post_obj
        .get("comments_url")
        .unwrap()
        .as_str()
        .unwrap()
        .to_owned();
    post.submitter_user = parse_user(user_obj);
    for tag in post_obj.get("tags").unwrap().as_array().unwrap() {
        tags.push(tag.as_str().unwrap().to_owned())
    }
    post.tags = Some(tags);
    post
}

fn parse_user(user_obj: &Value) -> User {
    let mut user: User = User::default();
    user.username = user_obj
        .get("username")
        .unwrap()
        .as_str()
        .unwrap()
        .to_owned();
    user.created_at = user_obj
        .get("created_at")
        .unwrap()
        .as_str()
        .unwrap()
        .to_owned();
    user.is_admin = user_obj.get("is_admin").unwrap().as_bool().unwrap();
    match user_obj.get("about") {
        Some(about) => {
            user.about = Some(about.as_str().unwrap().to_owned());
        }
        None => {}
    };
    user.is_moderator = user_obj.get("is_moderator").unwrap().as_bool().unwrap();
    match user_obj.get("karma") {
        Some(karma) => {
            user.karma = Some(karma.as_i64().unwrap() as i32);
        }
        None => {}
    };
    match user_obj.get("avtar_url") {
        Some(avtar_url) => {
            user.avtar_url = Some(avtar_url.as_str().unwrap().to_owned());
        }
        None => {}
    };
    user.invited_by_user = user_obj
        .get("invited_by_user")
        .unwrap()
        .as_str()
        .unwrap()
        .to_owned();
    match user_obj.get("github_username") {
        Some(github_username) => {
            user.github_username = Some(github_username.as_str().unwrap().to_owned());
        }
        None => {}
    };
    match user_obj.get("twitter_username") {
        Some(twitter_username) => {
            user.twitter_username = Some(twitter_username.as_str().unwrap().to_owned());
        }
        None => {}
    };
    user
}

#[cfg(test)]
mod url_gen_tests {
    use super::{generate_url, LobstersPath};
    #[test]
    fn generate_url_newest() {
        assert_eq!(
            generate_url(LobstersPath::Newest, None),
            "https://lobste.rs/newest.json"
        );
        assert_eq!(
            generate_url(LobstersPath::Newest, Some(654)),
            "https://lobste.rs/newest/page/654.json"
        );
        assert_eq!(
            generate_url(LobstersPath::Newest, Some(1u32)),
            "https://lobste.rs/newest/page/1.json"
        );
        assert_eq!(
            generate_url(LobstersPath::Newest, Some(599u32)),
            "https://lobste.rs/newest/page/599.json"
        );
    }

    #[test]
    fn generate_url_hottest() {
        assert_eq!(
            generate_url(LobstersPath::Hottest, None),
            "https://lobste.rs/hottest.json"
        );
        assert_eq!(
            generate_url(LobstersPath::Hottest, Some(6584)),
            "https://lobste.rs/hottest.json?page=6584"
        );
        assert_eq!(
            generate_url(LobstersPath::Hottest, Some(49u32)),
            "https://lobste.rs/hottest.json?page=49"
        );
        assert_eq!(
            generate_url(LobstersPath::Hottest, Some(3620)),
            "https://lobste.rs/hottest.json?page=3620"
        );
    }
}
