use crate::db::models::User;
use diesel::QueryResult;
use serde::Deserialize;
use actix::Message;


#[derive(Deserialize)]
pub struct UpdateUserForm{
    pub name: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub is_staff: Option<bool>,
}


#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<User>")]
pub struct LoginForm {
    pub name: String,
    pub password: String,
}
