use crate::apps::users::responses::UserPublic;
use apistos::ApiComponent;
use diesel::QueryResult;
use schemars::JsonSchema;
use serde::Deserialize;
use actix::Message;


#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct UpdateUserForm{
    pub name: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub is_staff: Option<bool>,
}

#[derive(Message, Deserialize, JsonSchema, ApiComponent)]
#[rtype(result = "QueryResult<UserPublic>")]
pub struct LoginForm {
    pub name: String,
    pub password: String,
}

#[derive(Message, Deserialize, JsonSchema, ApiComponent)]
#[rtype(result = "QueryResult<UserPublic>")]
pub struct CreateUser {
    pub name: String,
    pub password: String,
    pub email: Option<String>,
    pub is_staff: bool,
}
