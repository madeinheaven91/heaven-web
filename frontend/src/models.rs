use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug, Default)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub body: String,
    pub author: User,
    pub is_published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub tags: Vec<Tag>
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug, Default)]
pub struct Tag {
    pub slug: String,
    pub name: String,
    pub background_color: String,
    pub foreground_color: String,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug, Default, Properties)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub is_staff: bool,
}
