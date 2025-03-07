use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::{apps::users::responses::UserPublic, db::models::Tag};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct PostPublic {
    pub slug: String,
    pub title: String,
    pub body: String,
    pub author: UserPublic,
    pub is_published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub tags: Vec<Tag>
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct TagToPostPublic {
    pub post_slug: String,
    pub tag_slug: String
}
