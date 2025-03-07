use chrono::NaiveDateTime;
use diesel::Insertable;
use crate::db::schema::{posts, tags, tags_to_posts};
use serde::Serialize;

#[derive(Insertable, Debug, Serialize, Clone)]
#[diesel(table_name=posts)]
pub struct NewPost {
    pub slug: String,
    pub title: String,
    pub body: String,
    pub author_id: i32 
}

#[derive(AsChangeset)]
#[diesel(table_name=posts)]
pub struct PostUpdate {
    pub title: Option<String>,
    pub body: Option<String>,
    pub is_published: Option<bool>,
    pub updated_at: NaiveDateTime
}

#[derive(Insertable, Debug, Serialize, Clone)]
#[diesel(table_name=tags)]
pub struct NewTag {
    pub slug: String,
    pub name: String,
    pub background_color: Option<String>,
    pub foreground_color: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name=tags)]
pub struct TagUpdate {
    pub name: Option<String>,
    pub background_color: Option<String>,
    pub foreground_color: Option<String>,
}

#[derive(Insertable, Debug, Serialize, Clone)]
#[diesel(table_name=tags_to_posts)]
pub struct TagAssignment {
    pub post: String,
    pub tag: String
}
