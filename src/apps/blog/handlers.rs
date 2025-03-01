use actix_web::{web::{Data, Json, Path}, HttpMessage, HttpRequest, HttpResponse, Responder};

use crate::{db::AppState, shared::{statics::LEXICON, utils::Claims}};

use super::{forms::{PostCreateForm, PostUpdateForm}, messages::{CreatePost, DeletePost, GetPost, GetPosts, UpdatePost}};


pub async fn create_post(req: HttpRequest, state: Data<AppState>, body: Json<PostCreateForm>) -> impl Responder {
    let form = body.into_inner();
    let db = state.db.clone();

    let post = if let Some(claims) = req.extensions().get::<Claims>() {
        CreatePost {
            title: form.title.clone(),
            body: form.body.clone(),
            author_id: claims.sub
        }
    }else{
        return HttpResponse::Unauthorized().finish();
    };
    match db.send(post).await {
        Ok(Ok(res)) => HttpResponse::Ok().json(res),
        _ => HttpResponse::InternalServerError().body(LEXICON["db_error"]),
    }
}

pub async fn get_posts(state: Data<AppState>) -> impl Responder {
    let db = state.db.clone();

    match db.send(GetPosts).await {
        Ok(Ok(res)) => HttpResponse::Ok().json(res),
        _ => HttpResponse::InternalServerError().body(LEXICON["db_error"]),
    }
}

pub async fn get_post(state: Data<AppState>, path: Path<String>) -> impl Responder {
    let slug = path.into_inner();
    let db = state.db.clone();

    match db.send(GetPost {slug}).await {
        Ok(Ok(res)) => HttpResponse::Ok().json(res),
        _ => HttpResponse::InternalServerError().body(LEXICON["db_error"]),
    }
}

pub async fn update_post(req: HttpRequest, state: Data<AppState>, path: Path<String>, body: Json<PostUpdateForm>) -> impl Responder {
    let slug = path.into_inner();
    let form = body.into_inner();
    let db = state.db.clone();

    let post = match db.send(GetPost {slug: slug.clone()}).await {
        Ok(query_res) => match query_res {
            Ok(post) => post,
            Err(_) => return HttpResponse::NotFound().body("Post not found"),
        },
        _ => return HttpResponse::InternalServerError().body(LEXICON["db_error"])
    };
    if let Some(claims) = req.extensions().get::<Claims>() {
        if claims.sub != post.author_id {
            return HttpResponse::Forbidden().finish();
        }
    } else {
        return HttpResponse::Unauthorized().finish();
    };

    let post = UpdatePost {
        slug,
        title: form.title,
        body: form.body,
        is_published: form.is_published
    };
    match db.send(post).await {
        Ok(Ok(res)) => HttpResponse::Ok().json(res),
        _ => HttpResponse::InternalServerError().body(LEXICON["db_error"]),
    }
}


pub async fn delete_post(req: HttpRequest, state: Data<AppState>, path: Path<String>) -> impl Responder {
    let slug = path.into_inner();
    let db = state.db.clone();

    let post = match db.send(GetPost {slug: slug.clone()}).await {
        Ok(query_res) => match query_res {
            Ok(post) => post,
            Err(_) => return HttpResponse::NotFound().body("Post not found"),
        },
        _ => return HttpResponse::InternalServerError().body(LEXICON["db_error"])
    };
    if let Some(claims) = req.extensions().get::<Claims>() {
        if claims.sub != post.author_id {
            return HttpResponse::Forbidden().finish();
        }
    } else {
        return HttpResponse::Unauthorized().finish();
    };

    match db.send(DeletePost {slug}).await {
        Ok(Ok(res)) => HttpResponse::Ok().json(res),
        _ => HttpResponse::InternalServerError().body(LEXICON["db_error"]),
    }
}
