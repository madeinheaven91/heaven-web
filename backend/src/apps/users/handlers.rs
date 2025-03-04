
use super::{
    forms::{LoginForm, UpdateUserForm, CreateUser},
    messages::{DeleteUser, FetchUser, FetchUsers, UpdateUser},
};
use crate::{
    db::AppState,
    shared::{
        statics::LEXICON, utils::{create_access_token, create_refresh_token, get_and_send_back, Claims}
    },
};
use actix_web::{
    web::{Data, Json, Path},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};
use apistos::api_operation;
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
    if let Some(claims) = req.extensions().get::<Claims>() {
        if claims.sub != id && !claims.staff {
            return HttpResponse::Forbidden().finish();
        }
    } else {
        return HttpResponse::Unauthorized().finish();
    };
    let user = body.into_inner();
    let db = state.db.clone();
    let msg = UpdateUser {
            id,
            name: user.name,
            password: user.password,
            email: user.email,
            is_staff: user.is_staff,
        };
    get_and_send_back(db, msg).await
}

/// Delete a user and return it
#[api_operation(tag = "users", security_scope(name = "jwt token", scope = "write:users"))]
pub async fn delete_user(req: HttpRequest, state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id = path.into_inner();
    if let Some(claims) = req.extensions().get::<Claims>() {
        if claims.sub != id && !claims.staff {
            return HttpResponse::Forbidden().finish();
        }
    } else {
        return HttpResponse::Unauthorized().finish();
    };
    let db = state.db.clone();
    let msg = DeleteUser{id};
    
    get_and_send_back(db, msg).await
}

/// Login
/// Returns a response with Access and Refresh tokens
#[api_operation(tag = "users")]
pub async fn login(state: Data<AppState>, body: Json<LoginForm>) -> impl Responder {
    let db = state.db.clone();
    let msg = body.into_inner();

    match db.send(msg).await {
        Ok(res) => match res {
            Ok(user) => {
                let access = create_access_token(user.id, user.is_staff);
                let refresh = create_refresh_token(user.id, user.is_staff);
                // HttpResponse::Ok().append_header(("Authorization", format!("Bearer {}", token).as_str())).finish()
                HttpResponse::Ok().json(json!({"access": access, "refresh": refresh}))
            }
            _ => HttpResponse::Unauthorized().body("Wrong name or password"),
        },
        _ => HttpResponse::InternalServerError().body(LEXICON["mailbox_error"]),
    }
}

#[allow(clippy::empty_line_after_outer_attr)]
// TODO:
/// Refresh
// #[api_operation(tag = "users")]
// pub async fn refresh_token(req: HttpRequest) -> impl Responder {
//     let token = if let Some(claims) = req.extensions().get::<Claims>() {
//         json!(claims).to_string()
//     }else{
//         return HttpResponse::Unauthorized().finish();
//     };
//     match refresh(&token) {
//         Ok(access) => HttpResponse::Ok().json(json!({"access": access})),
//         _ => HttpResponse::Unauthorized().finish()
//     }
// }

/// Profile
/// Returns user data extracted from bearer token
#[api_operation(tag = "users", security_scope(name = "jwt token", scope = "read:users"))]
pub async fn profile(state: Data<AppState>, req: HttpRequest) -> impl Responder {
    let msg = if let Some(claims) = req.extensions().get::<Claims>() {
        FetchUser {
            id: claims.sub
        }
    }else{
        return HttpResponse::Unauthorized().finish();
    };
    let db = state.db.clone();
    get_and_send_back(db, msg).await
}

