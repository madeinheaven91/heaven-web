use chrono::NaiveDateTime;
use serde::Serialize;
use crate::{apps::users::responses::UserPublic, db::models::Tag};

#[derive(Queryable, Serialize, Debug)]
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
