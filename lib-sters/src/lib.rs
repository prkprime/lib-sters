pub mod models;
use minreq::{self, Response};
use models::{post::Post, user::User};
use serde_json::Value;

pub fn generate_url(path: &str, page: Option<u32>) -> Option<String> {
    let mut url: String = "https://lobste.rs/".to_owned();
    if path == "newest" {
        url = format!("{}newest", url);
        match page {
            Some(page) => url = format!("{}/page/{}", url, page),
            None => {}
        }
        url = format!("{}.json", url);
        Some(url)
    } else if path == "hottest" {
        url = format!("{}hottest", url);
        url = format!("{}.json", url);
        match page {
            Some(page) => url = format!("{}?page={}", url, page),
            None => {}
        }
        Some(url)
    } else {
        None
    }
}

pub fn generate_posts(url: String) -> Vec<Post> {
    let responce: Response = minreq::get(url).send().unwrap();
    let res_str: &str = responce.as_str().unwrap();
    let json_value: Value = serde_json::from_str(res_str).unwrap();
    let obj_vec: &Vec<Value> = json_value.as_array().unwrap();
    let mut posts: Vec<Post> = Vec::new();
    for post_obj in obj_vec {
        let mut post: Post = Post::default();
        let mut user: User = User::default();
        let mut tags: Vec<String> = Vec::new();
        for user_obj in post_obj.get("submitter_user") {
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
        }
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
        post.submitter_user = user;
        for tag in post_obj.get("tags").unwrap().as_array().unwrap() {
            tags.push(tag.as_str().unwrap().to_owned())
        }
        post.tags = Some(tags);
        posts.push(post);
    }
    posts
}

#[test]
fn test_get_posts() {
    for post in generate_posts(generate_url("hottest", None).unwrap()) {
        println!("{:?}", post)
    }
}
