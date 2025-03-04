use actix_web::{web::{Data, Json, Path}, HttpMessage, HttpRequest, HttpResponse, Responder};
use apistos::api_operation;

use crate::{db::AppState, shared::{errors::APIError, utils::{get_and_send_back, get_claims, Claims}}};

use super::{forms::{PostCreateForm, PostUpdateForm, TagCreateForm, TagUpdateForm}, messages::{CreatePost, CreateTag, DeletePost, DeleteTag, GetPost, GetPosts, GetTag, GetTags, UpdatePost, UpdateTag}};

// ------------------------- //
//          Posts            //
// ------------------------- //

/// Create a post and return it
#[api_operation(tag = "posts", security_scope(name = "jwt token", scope = "write:posts"))]
pub async fn create_post(req: HttpRequest, state: Data<AppState>, body: Json<PostCreateForm>) -> impl Responder {
    let form = body.into_inner();
    let db = state.db.clone();
    let msg = match get_claims(req, "access_token").await {
        Ok(claims) => CreatePost {
            title: form.title.clone(),
            body: form.body.clone(),
            author_id: claims.sub
        },
        Err(err) => return err.to_http()
    };

    get_and_send_back(db, msg).await
}

/// Fetch all posts
#[api_operation(tag = "posts")]
pub async fn get_posts(state: Data<AppState>) -> impl Responder {
    let db = state.db.clone();
    let msg = GetPosts;
    get_and_send_back(db, msg).await
}

/// Fetch a post by ID
#[api_operation(tag = "posts")]
pub async fn get_post(state: Data<AppState>, path: Path<String>) -> impl Responder {
    let slug = path.into_inner();
    let db = state.db.clone();
    let msg = GetPost{slug};
    get_and_send_back(db, msg).await
}

/// Update a post by ID
#[api_operation(tag = "posts", security_scope(name = "jwt token", scope = "write:posts"))]
pub async fn update_post(req: HttpRequest, state: Data<AppState>, path: Path<String>, body: Json<PostUpdateForm>) -> impl Responder {
    let slug = path.into_inner();
    let form = body.into_inner();
    let db = state.db.clone();
    let post = match db.send(GetPost {slug: slug.clone()}).await {
        Ok(query_res) => match query_res {
            Ok(post) => post,
            Err(_) => return HttpResponse::NotFound()
                .content_type("text/html")
                .body("Post not found"),
        },
        _ => return APIError::Forbidden.to_http()
    };
    let msg = match get_claims(req, "access_token").await{
        Ok(claims) => {
            if claims.sub != post.author_id && !claims.staff {
                return APIError::Forbidden.to_http()
            };
            UpdatePost {
                slug,
                title: form.title,
                body: form.body,
                is_published: form.is_published
            }
        },
        Err(err) => return err.to_http(),
    };

    get_and_send_back(db, msg).await
}


/// Delete a post by ID and return it
#[api_operation(tag = "posts", security_scope(name = "jwt token", scope = "write:posts"))]
pub async fn delete_post(req: HttpRequest, state: Data<AppState>, path: Path<String>) -> impl Responder {
    let slug = path.into_inner();
    let db = state.db.clone();
    let post = match db.send(GetPost {slug: slug.clone()}).await {
        Ok(query_res) => match query_res {
            Ok(post) => post,
            Err(_) => return HttpResponse::NotFound()
                .content_type("text/html")
                .body("Post not found"),
        },
        _ => return APIError::Forbidden.to_http()
    };
    let msg = match get_claims(req, "access_token").await{
        Ok(claims) => {
            if claims.sub != post.author_id && !claims.staff {
                return APIError::Forbidden.to_http()
            };
            DeletePost{slug}
        },
        Err(err) => return err.to_http(),
    };

    get_and_send_back(db, msg).await
}

// ------------------------- //
//          Tags             //
// ------------------------- //

/// Create a tag and return it
#[api_operation(tag = "tags", security_scope(name = "admin jwt token", scope = "write:tags"))]
pub async fn create_tag(req: HttpRequest, state: Data<AppState>, body: Json<TagCreateForm>) -> impl Responder {
    let form = body.into_inner();
    let db = state.db.clone();

    // let msg = if let Some(claims) = req.extensions().get::<Claims>() {
    //     if !claims.staff{
    //         return HttpResponse::Forbidden().finish();
    //     }
    //     else{
    //         CreateTag {
    //             name: form.name,
    //             background_color: form.background_color,
    //             foreground_color: form.foreground_color
    //         }
    //     }
    // }else{
    //     return HttpResponse::Unauthorized().finish();
    // };
    let msg = match get_claims(req, "access_token").await {
        Ok(claims) => {
            if !claims.staff{
                return APIError::Forbidden.to_http()
            };
            CreateTag {
                name: form.name,
                background_color: form.background_color,
                foreground_color: form.foreground_color
            }
        },
        Err(err) => return err.to_http()
    };
    get_and_send_back(db, msg).await
}

/// Fetch all tags
#[api_operation(tag = "tags")]
pub async fn get_tags(state: Data<AppState>) -> impl Responder {
    let db = state.db.clone();
    let msg = GetTags;
    get_and_send_back(db, msg).await
}

/// Fetch a tag by ID
#[api_operation(tag = "tags")]
pub async fn get_tag(state: Data<AppState>, path: Path<String>) -> impl Responder {
    let slug = path.into_inner();
    let db = state.db.clone();
    let msg = GetTag{slug};
    get_and_send_back(db, msg).await
}

/// Update a tag and return it
#[api_operation(tag = "tags", security_scope(name = "admin jwt token", scope = "write:tags"))]
pub async fn update_tag(req: HttpRequest, state: Data<AppState>, path: Path<String>, body: Json<TagUpdateForm>) -> impl Responder {
    let slug = path.into_inner();
    let form = body.into_inner();
    let db = state.db.clone();

    // let tag = match db.send(GetTag {slug: slug.clone()}).await {
    //     Ok(query_res) => match query_res {
    //         Ok(tag) => tag,
    //         Err(_) => return HttpResponse::NotFound().body("tag not found"),
    //     },
    //     _ => return HttpResponse::InternalServerError().body(LEXICON["db_error"])
    // };
    if let Some(claims) = req.extensions().get::<Claims>() {
        if !claims.staff {
            return HttpResponse::Forbidden().finish();
        }
    } else {
        return HttpResponse::Unauthorized().finish();
    };

    let msg = UpdateTag {
        slug,
        name: form.name,
        background_color: form.background_color,
        foreground_color: form.foreground_color
    };
    get_and_send_back(db, msg).await
}


/// Delete a tag and return it
#[api_operation(tag = "tags", security_scope(name = "admin jwt token", scope = "write:tags"))]
pub async fn delete_tag(req: HttpRequest, state: Data<AppState>, path: Path<String>) -> impl Responder {
    let slug = path.into_inner();
    let db = state.db.clone();
    let msg = match get_claims(req, "access_token").await{
        Ok(claims) => {
            if !claims.staff {
                return APIError::Forbidden.to_http()
            };
            DeleteTag{slug}
        },
        Err(err) => return err.to_http(),
    };

    get_and_send_back(db, msg).await
}

