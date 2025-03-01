use chrono::NaiveDateTime;
use diesel::Insertable;
use crate::db::schema::posts;
use serde::Serialize;

#[derive(Insertable, Debug, Serialize, Clone)]
#[table_name="posts"]
pub struct NewPost {
    pub slug: String,
    pub title: String,
    pub body: String,
    pub author_id: i32 
}

#[derive(AsChangeset)]
#[table_name = "posts"]
pub struct PostUpdate {
    pub title: Option<String>,
    pub body: Option<String>,
    pub is_published: Option<bool>,
    pub updated_at: NaiveDateTime
}

