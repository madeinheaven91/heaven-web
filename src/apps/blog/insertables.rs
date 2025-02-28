use diesel::Insertable;
use crate::db::schema::posts;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Debug, Serialize, Clone)]
#[table_name="posts"]
pub struct NewPost {
    pub slug: String,
    pub title: String,
    pub body: String,
    pub author_id: i32 
}

#[derive(AsChangeset, Deserialize)]
#[table_name = "posts"]
pub struct PostUpdateForm {
    pub title: Option<String>,
    pub body: Option<String>,
    pub is_published: Option<bool>
}
