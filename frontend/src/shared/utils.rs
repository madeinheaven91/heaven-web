use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use crate::models::{Post, Tag, User};

use super::statics::CONFIG;
use reqwest;

pub async fn get_profile() -> Result<User, reqwest::Error>{
    let token = get_access_token().unwrap_or_default();
    let client = reqwest::Client::new();
    let resp = client.get(format!("{}/users/profile", CONFIG.api_url()))
        .bearer_auth(token)
        .send()
        .await?
        .text()
        .await?;
    Ok(serde_json::from_str(&resp).unwrap())
}

pub async fn get_users() -> Result<Vec<User>, reqwest::Error>{
    let body = reqwest::get(&format!("{}/users/fetch", CONFIG.api_url()))
        .await?
        .text()
        .await?;
    Ok(serde_json::from_str(&body).unwrap())
}

pub async fn get_posts() -> Result<Vec<Post>, reqwest::Error>{
    let body = reqwest::get(&format!("{}/blog/posts/fetch", CONFIG.api_url()))
        .await?
        .text()
        .await?;
    Ok(serde_json::from_str(&body).unwrap())
}

pub async fn get_tags() -> Result<Vec<Tag>, reqwest::Error>{
    let body = reqwest::get(&format!("{}/blog/tags/fetch", CONFIG.api_url()))
        .await?
        .text()
        .await?;
    Ok(serde_json::from_str(&body).unwrap())
}

pub async fn get_user(id: i32) -> Result<User, reqwest::Error>{
    let body = reqwest::get(&format!("{}/users/fetch/{id}", CONFIG.api_url()))
        .await?
        .text()
        .await?;
    Ok(serde_json::from_str(&body).unwrap())
}

pub async fn get_post(slug: String) -> Result<Post, reqwest::Error>{
    let body = reqwest::get(&format!("{}/blog/posts/fetch/{slug}", CONFIG.api_url()))
        .await?
        .text()
        .await?;
    Ok(serde_json::from_str(&body).unwrap())
}

pub async fn get_tag(slug: String) -> Result<Tag, reqwest::Error>{
    let body = reqwest::get(&format!("{}/blog/tags/fetch/{slug}", CONFIG.api_url()))
        .await?
        .text()
        .await?;
    Ok(serde_json::from_str(&body).unwrap())

}

pub async fn logout() -> Result<(), String> {
    remove_access_token();
    let client = reqwest::ClientBuilder::new()
        // .cookie_store(true)
        .build()
        .map_err(|err| err.to_string())?;
    let _ = client.get(format!("{}/users/logout", CONFIG.api_url()))
        .bearer_auth(get_access_token().unwrap_or_default())
        .send()
        .await
        .map_err(|err| err.to_string())?;
    Ok(())
}

pub async fn login(username: &str, password: &str) -> Result<String, String> {
    let body = format!("name={}&password={}", username, password);
    let client = reqwest::ClientBuilder::new()
        // .cookie_store(true)
        .build()
        .map_err(|err| err.to_string())?;
    let builder = client.post(format!("{}/users/login", CONFIG.api_url()))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body);
    let response = builder
        .send()
        .await
        .map_err(|err| err.to_string())?;
    let token = serde_json::from_str::<Token>(&response.text().await.map_err(|err| err.to_string())?).unwrap().access_token;
    set_access_token(token.clone());
    Ok(token)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Token {
    access_token: String
}

impl Token {
    pub fn access(&self) -> &String {
        &self.access_token
    }
}

pub fn set_access_token(token: String) {
    LocalStorage::set("access_token", token).unwrap()
}

pub fn get_access_token() -> Option<String> {
    LocalStorage::get("access_token").ok()
}

pub fn remove_access_token() {
    LocalStorage::delete("access_token")
}
