use actix_web::{
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde::Deserialize;
use crate::{db::pg::AppState, shared::LEXICON};
use super::messages::{CreateUser, DeleteUser, FetchUser, FetchUsers, UpdateUser};

pub async fn new_user(state: Data<AppState>, body: Json<CreateUser>) -> impl Responder {
    let user = body.into_inner();
    let db = state.db.clone();
    match db.send(user).await {
        Ok(Ok(res)) => HttpResponse::Ok().json(res),
        _ => HttpResponse::InternalServerError().json(LEXICON["db_error"]),
    }
}

pub async fn fetch_users(state: Data<AppState>) -> impl Responder {
    let db = state.db.clone();
    match db.send(FetchUsers).await {
        Ok(Ok(res)) => HttpResponse::Ok().json(res),
        _ => HttpResponse::InternalServerError().json(LEXICON["db_error"]),
    }
}

pub async fn fetch_user(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let db = state.db.clone();
    match db
        .send(FetchUser {
            id: path.into_inner(),
        })
        .await
    {
        Ok(Ok(res)) => HttpResponse::Ok().json(res),
        _ => HttpResponse::InternalServerError().json(LEXICON["db_error"]),
    }
}

#[derive(Deserialize)]
pub struct UpdateUserNoId{
    name: String,
    password: String,
    email: Option<String>,
    is_staff: bool
}

pub async fn update_user(state: Data<AppState>, path: Path<i32>, body: Json<UpdateUserNoId>) -> impl Responder {
    let user = body.into_inner();
    let db = state.db.clone();
    match db.send(UpdateUser{
        id: path.into_inner(),
        name: user.name,
        password: user.password,
        email: user.email,
        is_staff: user.is_staff,
    }).await {
        Ok(Ok(res)) => HttpResponse::Ok().json(res),
        _ => HttpResponse::InternalServerError().json(LEXICON["db_error"]),
    }
}

pub async fn delete_user(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let db = state.db.clone();
    match db
        .send(DeleteUser {
            id: path.into_inner(),
        })
        .await
    {
        Ok(Ok(res)) => HttpResponse::Ok().json(res),
        _ => HttpResponse::InternalServerError().json(LEXICON["db_error"]),
    }
}
