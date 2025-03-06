use gloo::console;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use wasm_cookies::cookies::get_raw;
use web_sys::{wasm_bindgen::JsCast, HtmlDocument, RequestCredentials};
use crate::models::{Post, Tag, User};

pub async fn get_users() -> Result<Vec<User>, gloo_net::Error>{
    Request::get("http://localhost:8000/api/v1/users")
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn get_posts() -> Result<Vec<Post>, gloo_net::Error>{
    Request::get("http://localhost:8000/api/v1/blog/posts")
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn get_tags() -> Result<Vec<Tag>, gloo_net::Error>{
    Request::get("http://localhost:8000/api/v1/blog/tags")
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn get_user(id: i32) -> Result<User, gloo_net::Error>{
    Request::get(format!("http://localhost:8000/api/v1/users/{id}").as_str())
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn get_post(slug: String) -> Result<Post, gloo_net::Error>{
    Request::get(format!("http://localhost:8000/api/v1/blog/posts/{slug}").as_str())
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn get_tag(slug: String) -> Result<Tag, gloo_net::Error>{
    Request::get(format!("http://localhost:8000/api/v1/blog/tags/{slug}").as_str())
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub fn logged_in() -> bool {
    let cookies = gloo::utils::document().unchecked_into::<HtmlDocument>().cookie().unwrap();
    console::log!(cookies.clone());
    get_raw(&cookies, "access_token").is_some()
}

pub fn logout() {
    spawn_local(async {
    let _ = Request::get("http://localhost:8000/api/v1/users/logout")
        .credentials(RequestCredentials::Include)
        .send()
        .await;
    });
}

// pub const ACCESS_TOKEN_KEY: &str = "access_token";
// pub const REFRESH_TOKEN_KEY: &str = "refresh_token";
//
// #[derive(Deserialize, Debug)]
// pub struct Tokens {
//     pub access_token: String,
//     pub refresh_token: String
// }
//
// pub async fn save_tokens(access_token: &str, refresh_token: &str) {
//     LocalStorage::set(ACCESS_TOKEN_KEY, access_token).expect("Failed to save access token");
//     LocalStorage::set(REFRESH_TOKEN_KEY, refresh_token).expect("Failed to save refresh token");
// }
//
// pub fn delete_tokens() {
//     LocalStorage::delete(ACCESS_TOKEN_KEY);
//     LocalStorage::delete(REFRESH_TOKEN_KEY);
// }
//
// pub fn get_access_token() -> Option<String> {
//     LocalStorage::get(ACCESS_TOKEN_KEY).ok()
// }
//
// pub fn get_refresh_token() -> Option<String> {
//     LocalStorage::get(REFRESH_TOKEN_KEY).ok()
// }
