use super::{
    forms::{LoginForm, UpdateUserForm},
    messages::{CreateUser, DeleteUser, FetchUser, FetchUsers, UpdateUser},
};
use crate::{
    db::AppState,
    shared::{
        statics::LEXICON,
        utils::{create_jwt, Claims},
    },
};
use actix_web::{
    web::{Data, Json, Path},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};

pub async fn new_user(state: Data<AppState>, body: Json<CreateUser>) -> impl Responder {
    let user = body.into_inner();
    let db = state.db.clone();
    match db.send(user).await {
        Ok(Ok(res)) => HttpResponse::Ok().json(res),
        _ => HttpResponse::InternalServerError().body(LEXICON["db_error"]),
    }
}

pub async fn fetch_users(state: Data<AppState>) -> impl Responder {
    let db = state.db.clone();
    match db.send(FetchUsers).await {
        Ok(Ok(res)) => HttpResponse::Ok().json(res),
        _ => HttpResponse::InternalServerError().body(LEXICON["db_error"]),
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
        _ => HttpResponse::InternalServerError().body(LEXICON["db_error"]),
    }
}

pub async fn update_user(
    req: HttpRequest,
    state: Data<AppState>,
    path: Path<i32>,
    body: Json<UpdateUserForm>,
) -> impl Responder {
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
    match db
        .send(UpdateUser {
            id,
            name: user.name,
            password: user.password,
            email: user.email,
            is_staff: user.is_staff,
        })
        .await
    {
        Ok(Ok(res)) => HttpResponse::Ok().json(res),
        _ => HttpResponse::InternalServerError().body(LEXICON["db_error"]),
    }
}

pub async fn delete_user(
    req: HttpRequest,
    state: Data<AppState>,
    path: Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    if let Some(claims) = req.extensions().get::<Claims>() {
        if claims.sub != id && !claims.staff {
            return HttpResponse::Forbidden().finish();
        }
    } else {
        return HttpResponse::Unauthorized().finish();
    };
    let db = state.db.clone();
    match db.send(DeleteUser { id }).await {
        Ok(Ok(res)) => HttpResponse::Ok().json(res),
        _ => HttpResponse::InternalServerError().body(LEXICON["db_error"]),
    }
}

pub async fn login(state: Data<AppState>, body: Json<LoginForm>) -> impl Responder {
    let user = body.into_inner();
    let db = state.db.clone();
    match db.send(user).await {
        Ok(res) => match res {
            Ok(user) => {
                let token = create_jwt(user.id, user.is_staff);
                HttpResponse::Ok().body(token)
            }
            _ => HttpResponse::Unauthorized().body("Wrong name or password"),
        },
        _ => HttpResponse::InternalServerError().body(LEXICON["db_error"]),
    }
}
