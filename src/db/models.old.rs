use serde::{Deserialize, Serialize};

use super::schema::{users, tags, posts, tags_to_posts};

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser{
    pub name: String,
    pub password: String,
    pub email: String,
    pub is_staff: bool
}

#[derive(Debug, Queryable, AsChangeset)]
pub struct User{
    pub id: i32,
    pub name: String,
    pub password: String,
    pub email: String,
    pub is_staff: bool,
}

#[derive(Debug, Queryable, AsChangeset)]
#[table_name = "tags"]
pub struct Tag{
    pub id: i32,
    pub name: String,
    pub background_color: String,
    pub foreground_color: String
}


