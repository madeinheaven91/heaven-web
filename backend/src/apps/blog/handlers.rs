use crate::apps::blog::responses::TagToPostPublic;
use crate::apps::blog::messages::RemoveTagFromPost;
use crate::apps::blog::messages::AssignTagToPost;
use actix::Addr;
use actix_web::{web::{Data, Json, Path}, HttpRequest, HttpResponse, Responder};
use apistos::api_operation;

use crate::{apps::users::{messages::FetchUser, responses::UserPublic}, db::{models::Post, AppState, DbActor}, shared::{errors::APIError, utils::{get_and_send_back, get_claims_by_auth, get_from_db}}};

use super::{forms::{PostCreateForm, PostUpdateForm, TagCreateForm, TagUpdateForm}, messages::{CreatePost, CreateTag, DeletePost, DeleteTag, GetPost, GetPosts, GetTag, GetTags, UpdatePost, UpdateTag, GetPostTags}};
use super::responses::PostPublic;

// ------------------------- //
//          Posts            //
// ------------------------- //

/// Create a post and return it
#[api_operation(tag = "posts", security_scope(name = "jwt token", scope = "write:posts"))]
pub async fn create_post(req: HttpRequest, state: Data<AppState>, body: Json<PostCreateForm>) -> impl Responder {
    let form = body.into_inner();
    let db = state.db.clone();
    let msg = match get_claims_by_auth(req).await {
        Ok(claims) => CreatePost {
            title: form.title.clone(),
            body: form.body.clone(),
            author_id: claims.sub
        },
        Err(err) => return err.to_http()
    };

    get_and_send_back(db, msg).await
}

async fn get_public_post(db: &Addr<DbActor>, post: Post) -> Result<PostPublic, APIError> {
    let author = match post.author_id {
        None => UserPublic::null(),
        Some(id) => get_from_db(db.clone(), FetchUser{id}).await?
    };
    let tags = get_from_db(db.clone(), GetPostTags{slug: post.slug.clone()}).await?;
    let res = PostPublic {
            tags,
            author,
            slug: post.slug,
            title: post.title,
            body: post.body,
            is_published: post.is_published,
            created_at: post.created_at,
            updated_at: post.updated_at,
    };
    Ok(res)
}

/// Fetch all posts
#[api_operation(tag = "posts")]
pub async fn get_posts(state: Data<AppState>) -> impl Responder {
    let db = state.db.clone();
    let msg = GetPosts;
    let posts = get_from_db(db.clone(), msg).await;
    let posts = match posts {
        Ok(res) => res,
        Err(err) => return err.to_http()
    };
    let mut res = vec![];
    for post in posts {
        let new = get_public_post(&db, post).await;
        match new {
            Ok(r) => res.push(r),
            Err(err) => return err.to_http()
        }
    };
    HttpResponse::Ok().json(res)
}

/// Fetch a post by ID
#[api_operation(tag = "posts")]
pub async fn get_post(state: Data<AppState>, path: Path<String>) -> impl Responder {
    let slug = path.into_inner();
    let db = state.db.clone();
    let msg = GetPost{slug};
    let post = get_from_db(db.clone(), msg).await;
    let post = match post {
        Ok(res) => res,
        Err(err) => return err.to_http()
    };
    let res = match get_public_post(&db, post).await {
        Ok(r) => r,
        Err(err) => return err.to_http()
    };
    HttpResponse::Ok().json(res)
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
    let msg = match get_claims_by_auth(req).await{
        Ok(claims) => {
            if claims.sub != post.author_id.unwrap_or(-1) && !claims.staff {
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

    let exec = get_from_db(db.clone(), msg).await;
    let updated = match exec {
        Ok(res) => res,
        Err(err) => return err.to_http()
    };

    match get_public_post(&db, updated).await {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(err) => err.to_http()
    }
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
    let msg = match get_claims_by_auth(req).await{
        Ok(claims) => {
            if claims.sub != post.author_id.unwrap_or(-1) && !claims.staff {
                return APIError::Forbidden.to_http()
            };
            DeletePost{slug}
        },
        Err(err) => return err.to_http(),
    };

    let res = match get_public_post(&db, post).await {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(err) => err.to_http()
    };
    get_and_send_back(db, msg).await;
    res

}

// ------------------------- //
//          Tags             //
// ------------------------- //

/// Create a tag and return it
#[api_operation(tag = "tags", security_scope(name = "admin jwt token", scope = "write:tags"))]
pub async fn create_tag(req: HttpRequest, state: Data<AppState>, body: Json<TagCreateForm>) -> impl Responder {
    let form = body.into_inner();
    let db = state.db.clone();

    let msg = match get_claims_by_auth(req).await {
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

    match get_claims_by_auth(req).await{
            Ok(claims) => 
                if !claims.staff {
                    return APIError::Forbidden.to_http()
                },
            Err(err) => return err.to_http(),
    };

    let msg = UpdateTag {
        slug: slug.clone(),
        name: form.name,
        background_color: form.background_color,
        foreground_color: form.foreground_color
    };
    get_and_send_back(db.clone(), msg).await;
    get_and_send_back(db, GetTag{slug}).await
}


/// Delete a tag and return it
#[api_operation(tag = "tags", security_scope(name = "admin jwt token", scope = "write:tags"))]
pub async fn delete_tag(req: HttpRequest, state: Data<AppState>, path: Path<String>) -> impl Responder {
    let slug = path.into_inner();
    let db = state.db.clone();
    let res = get_from_db(db.clone(), GetTag{slug: slug.clone()}).await;
    let res = match res{
        Ok(r) => r,
        Err(err) => return err.to_http()
    };
    let msg = match get_claims_by_auth(req).await{
        Ok(claims) => {
            if !claims.staff {
                return APIError::Forbidden.to_http()
            };
            DeleteTag{slug}
        },
        Err(err) => return err.to_http(),
    };

    get_and_send_back(db, msg).await;
    HttpResponse::Ok().json(res)
}

/// Assign tag to post
#[api_operation(tag = "tags", security_scope(name = "jwt token", scope = "write:tags"))]
pub async fn assign_tag_to_post(req: HttpRequest, state: Data<AppState>, path: Path<(String, String)>) -> impl Responder {
    let path = path.into_inner();
    let (post_slug, tag_slug) = (path.0, path.1);
    let db = state.db.clone();
    let post = match get_from_db(db.clone(), GetPost{slug: post_slug.clone()}).await {
        Ok(res) => res,
        Err(err) => return err.to_http()
    };
    let msg = match get_claims_by_auth(req).await{
        Ok(claims) => {
            if claims.sub != post.author_id.unwrap_or(-1) && !claims.staff {
                return APIError::Forbidden.to_http()
            };
            AssignTagToPost{post_slug, tag_slug: tag_slug.clone()}
        },
        Err(err) => return err.to_http(),
    };

    let exec = get_from_db(db, msg).await;
    match exec {
        Ok(res) => HttpResponse::Ok().json(TagToPostPublic {
            post_slug: res.slug,
            tag_slug
        }),
        Err(err) => err.to_http()
    }
}

/// Remove tag from post
#[api_operation(tag = "tags", security_scope(name = "jwt token", scope = "write:tags"))]
pub async fn rm_tag_from_post(req: HttpRequest, state: Data<AppState>, path: Path<(String, String)>) -> impl Responder {
    let path = path.into_inner();
    let (post_slug, tag_slug) = (path.0, path.1);
    let db = state.db.clone();
    let post = match get_from_db(db.clone(), GetPost{slug: post_slug.clone()}).await {
        Ok(res) => res,
        Err(err) => return err.to_http()
    };
    let msg = match get_claims_by_auth(req).await{
        Ok(claims) => {
            if claims.sub != post.author_id.unwrap_or(-1) && !claims.staff {
                return APIError::Forbidden.to_http()
            };
            RemoveTagFromPost{post_slug, tag_slug: tag_slug.clone()}
        },
        Err(err) => return err.to_http(),
    };

    let exec = get_from_db(db, msg).await;
    match exec {
        Ok(res) => HttpResponse::Ok().json(TagToPostPublic {
            post_slug: res.slug,
            tag_slug
        }),
        Err(err) => err.to_http()
    }
}
