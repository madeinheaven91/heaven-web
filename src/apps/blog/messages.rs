use actix::{Handler, Message};
use diesel::{QueryResult, RunQueryDsl};
use crate::db::{models::Post, pg::DbActor};
use serde::Deserialize;

#[derive(Message)]
#[rtype(result = "QueryResult<Post>")]
pub struct GetPost;

#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<Post>")]
pub struct CreatePost {

}

