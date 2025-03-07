use super::{
    forms::{LoginForm, UpdateUserForm, CreateUser},
    messages::{DeleteUser, FetchUser, FetchUsers, UpdateUser},
};
use crate::{
    db::AppState,
    shared::{
        errors::APIError, statics::{ACCESS_EXPIRATION, CONFIG, REFRESH_EXPIRATION}, utils::{create_jwt, get_and_send_back, get_claims_by_auth, get_from_db, hash_password, random_string, verify_jwt}
    },
};
use actix_web::{
    cookie::{self, time::OffsetDateTime, Cookie}, web::{Data, Form, Json, Path}, HttpRequest, HttpResponse, Responder
}; use apistos::api_operation;
use serde_json::json;

/// Create a new user and return it
#[api_operation(tag = "users")]
pub async fn new_user(state: Data<AppState>, body: Json<CreateUser>) -> impl Responder {
    let msg = body.into_inner();
    let db = state.db.clone();
    get_and_send_back(db, msg).await
}

/// Fetch all users
#[api_operation(tag = "users")]
pub async fn fetch_users(state: Data<AppState>) -> impl Responder {
    let db = state.db.clone();
    let msg = FetchUsers;
    get_and_send_back(db, msg).await
}

/// Fetch a user by ID
#[api_operation(tag = "users")]
pub async fn fetch_user(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let db = state.db.clone();
    let msg = FetchUser {
        id: path.into_inner()
    };
    get_and_send_back(db, msg).await
}

/// Update a user and return it
#[api_operation(tag = "users", security_scope(name = "jwt token", scope = "write:users"))]
pub async fn update_user(req: HttpRequest, state: Data<AppState>, path: Path<i32>, body: Json<UpdateUserForm>) -> impl Responder {
    let id = path.into_inner();
    let user = body.into_inner();
    let db = state.db.clone();
    let msg = match get_claims_by_auth(req).await{
        Err(err) => return err.to_http(),
        Ok(claims) => {
            if claims.sub != id && !claims.staff {
                return APIError::Forbidden.to_http()
            };
            UpdateUser {
                name: user.name,
                password: user.password.map(|str| hash_password(&str).unwrap()),
                id,
                is_staff: user.is_staff,
                email: user.email
            }
        }
    };
    get_and_send_back(db, msg).await
}

/// Delete a user and return it
#[api_operation(tag = "users", security_scope(name = "jwt token", scope = "write:users"))]
pub async fn delete_user(req: HttpRequest, state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let db = state.db.clone();
    let msg = match get_claims_by_auth(req).await{
        Err(err) => return err.to_http(),
        Ok(claims) => {
            if claims.sub != id && !claims.staff {
                return APIError::Forbidden.to_http()
            };
            DeleteUser {
                id
            }
        }
    };
    
    get_and_send_back(db, msg).await
}

/// Login
/// Returns a response with Access and Refresh tokens
#[api_operation(tag = "users")]
pub async fn login(state: Data<AppState>, body: Form<LoginForm>) -> impl Responder {
    let db = state.db.clone();
    let msg = body.into_inner();

    match db.send(msg).await {
        Ok(res) => match res {
            Ok(user) => {
                let access_token = create_jwt(user.id, user.is_staff, ACCESS_EXPIRATION);
                let refresh_token = create_jwt(user.id, user.is_staff, REFRESH_EXPIRATION);
                let refresh_cookie = match CONFIG.environment.dev() {
                    true => Cookie::build("refresh_token", &refresh_token)
                            .http_only(true)
                            .expires(OffsetDateTime::now_utc() + cookie::time::Duration::days(7))
                            .path("/")
                            .same_site(actix_web::cookie::SameSite::None)
                            .finish(),
                    false => Cookie::build("refresh_token", &refresh_token)
                            .http_only(true)
                            .expires(OffsetDateTime::now_utc() + cookie::time::Duration::days(7))
                            .path("/")
                            .same_site(actix_web::cookie::SameSite::None)
                            .secure(true)
                            .finish()
                };
                HttpResponse::Ok()
                    .cookie(refresh_cookie)
                    .json(json!({
                        "access_token": access_token
                    }))
            }
            _ => HttpResponse::Unauthorized()
                    .content_type("text/html")
                    .body("Wrong name or password"),
        },
        _ =>  APIError::MailboxError.to_http()
    }
}

#[allow(clippy::empty_line_after_outer_attr)]
/// Refresh
/// Gets refresh httponly cookie and returns a new access token
#[api_operation(tag = "users")]
pub async fn refresh_token(req: HttpRequest) -> impl Responder {
    let token = if let Some(cookie) = req.cookie("refresh_token") {
        cookie.value().to_string()
    }else{
        return APIError::MissingToken.to_http()
    };
    match verify_jwt(&token) {
        Ok(claims) => {
            let claims = claims.claims;
            let access_token = create_jwt(claims.sub, claims.staff, ACCESS_EXPIRATION);
            let refresh_token = create_jwt(claims.sub, claims.staff, REFRESH_EXPIRATION);
            let refresh_cookie = match CONFIG.environment.dev() {
                true => Cookie::build("refresh_token", &refresh_token)
                        .http_only(true)
                        .expires(OffsetDateTime::now_utc() + cookie::time::Duration::days(7))
                        .path("/")
                        .same_site(actix_web::cookie::SameSite::None)
                        .finish(),
                false => Cookie::build("refresh_token", &refresh_token)
                        .http_only(true)
                        .expires(OffsetDateTime::now_utc() + cookie::time::Duration::days(7))
                        .path("/")
                        .same_site(actix_web::cookie::SameSite::None)
                        .secure(true)
                        .finish()
            };
            HttpResponse::Ok()
                .cookie(refresh_cookie)
                .json(json!({
                    "access_token": access_token,
                    "refresh_token": refresh_token
                }))
        }
        _ => HttpResponse::Unauthorized().finish()
    }
}

/// Logout
/// Expires token cookie
#[api_operation(tag = "users")]
pub async fn logout(_: HttpRequest) -> impl Responder {
    let refresh = Cookie::build("refresh_token", "")
        .expires(OffsetDateTime::now_utc() - cookie::time::Duration::days(1))
        .path("/")
        .same_site(actix_web::cookie::SameSite::None)
        .finish();
    HttpResponse::Ok()
        .cookie(refresh)
        .json("Success")
}

/// Profile
/// Returns user data extracted from bearer token
#[api_operation(tag = "users", security_scope(name = "jwt token", scope = "read:users"))]
pub async fn profile(state: Data<AppState>, req: HttpRequest) -> impl Responder {
    let db = state.db.clone();
    let msg = match get_claims_by_auth(req).await{
        Err(err) => return err.to_http(),
        Ok(claims) => 
                FetchUser {
                    id: claims.sub
                }
    };
    get_and_send_back(db, msg).await
}

/// Superuser
/// Create a superuser and return its eternal access token
#[api_operation(tag = "users")]
pub async fn sudo(state: Data<AppState>) -> impl Responder {
    let msg = CreateUser {
        name: "superuser".to_string(),
        password: random_string(32),
        email: None,
        is_staff: true
    };
    let db = state.db.clone();
    let superuser = get_from_db(db, msg).await.expect("Failed to create superuser");
    HttpResponse::Ok()
        .json(json!({
            "access_token": create_jwt(superuser.id, true, 5259492)
        }))
}

