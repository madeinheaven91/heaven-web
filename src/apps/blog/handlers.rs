use actix_web::{web::{Data, Json, Path}, HttpResponse, Responder};

use crate::{db::pg::AppState, shared::LEXICON};

use super::{insertables::PostUpdateForm, messages::{CreatePost, DeletePost, GetPost, GetPosts, UpdatePost}};

pub async fn create_post(state: Data<AppState>, body: Json<CreatePost>) -> impl Responder {
    let post = body.into_inner();
    let db = state.db.clone();
    match db.send(post).await {
        Ok(Ok(res)) => HttpResponse::Ok().json(res),
        _ => HttpResponse::InternalServerError().json(LEXICON["db_error"]),
    }
}

pub async fn get_posts(state: Data<AppState>) -> impl Responder {
    let db = state.db.clone();
    match db.send(GetPosts).await {
        Ok(Ok(res)) => HttpResponse::Ok().json(res),
        _ => HttpResponse::InternalServerError().json(LEXICON["db_error"]),
    }
}

pub async fn get_post(state: Data<AppState>, path: Path<String>) -> impl Responder {
    let slug = path.into_inner();
    let db = state.db.clone();
    match db.send(GetPost {slug}).await {
        Ok(Ok(res)) => HttpResponse::Ok().json(res),
        _ => HttpResponse::InternalServerError().json(LEXICON["db_error"]),
    }
}

pub async fn update_post(state: Data<AppState>, path: Path<String>, body: Json<PostUpdateForm>) -> impl Responder {
    let form = body.into_inner();
    let post = UpdatePost {
        slug: path.into_inner(),
        title: form.title,
        body: form.body,
        is_published: form.is_published
    };
    let db = state.db.clone();
    match db.send(post).await {
        Ok(Ok(res)) => HttpResponse::Ok().json(res),
        _ => HttpResponse::InternalServerError().json(LEXICON["db_error"]),
    }
}


pub async fn delete_post(state: Data<AppState>, path: Path<String>) -> impl Responder {
    let slug = path.into_inner();
    let db = state.db.clone();
    match db.send(DeletePost {slug}).await {
        Ok(Ok(res)) => HttpResponse::Ok().json(res),
        _ => HttpResponse::InternalServerError().json(LEXICON["db_error"]),
    }
}
