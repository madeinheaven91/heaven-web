use diesel::Insertable;
use crate::db::schema::users;
use serde::Serialize;

#[derive(Insertable, Debug, Serialize, Clone)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub name: String,
    pub password: String,
    pub email: Option<String>,
    pub is_staff: bool,
}

#[derive(AsChangeset)]
#[diesel(table_name=users)]
pub struct UserUpdate {
    pub name: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub is_staff: Option<bool>,
}
