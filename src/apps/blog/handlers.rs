use actix_web::{web::{Data, Json}, HttpResponse, Responder};

use crate::db::pg::AppState;

// pub async fn get_posts(data: Data<AppState>) -> impl Responder {
//     let posts = data.db.send(GetPost {}).await.unwrap();
//     HttpResponse::Ok().json(posts)
// }
