use actix::{Addr, Handler, Message};
use actix_web::{HttpRequest, HttpResponse};
use argon2::{password_hash::{self, rand_core::OsRng, SaltString}, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{Duration, Utc};
use diesel::result::Error::NotFound;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::db::DbActor;

use super::{errors::APIError, statics::{CONFIG, LEXICON}};

const ACCESS_EXPIRATION: i64 = 15;  // Minutes
const REFRESH_EXPIRATION: i64 = 7 * 24 * 60;  // 7 days

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Claims{
    pub sub: i32,
    pub exp: i64,
    pub staff: bool
}

fn create_jwt(sub: i32, staff: bool, expiration_minutes: i64) -> String {
    let payload = Claims {
        sub,
        exp: (Utc::now() + Duration::minutes(expiration_minutes)).timestamp(),
        staff
    };
    encode(&Header::new(jsonwebtoken::Algorithm::HS512), &payload, &EncodingKey::from_secret(CONFIG.secret_key.as_bytes())).unwrap()
}

pub fn create_access_token(sub: i32, staff: bool) -> String {
    create_jwt(sub, staff, ACCESS_EXPIRATION)
}

pub fn create_refresh_token(sub: i32, staff: bool) -> String {
    create_jwt(sub, staff, REFRESH_EXPIRATION)
}

pub fn verify_jwt(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let validation = Validation::new(jsonwebtoken::Algorithm::HS512);
    decode::<Claims>(token, &DecodingKey::from_secret(CONFIG.secret_key.as_bytes()), &validation)
}

pub fn hash_password(password: &str) -> Result<String, password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(hashed_password)
}

pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    Argon2::default()
        .verify_password(password.as_bytes(), &PasswordHash::new(hashed_password).unwrap())
        .is_ok()
}

pub async fn get_and_send_back<M, T>(db: Addr<DbActor>, msg: M) -> HttpResponse
where
    M: Message<Result = Result<T, diesel::result::Error>> + Send + 'static,
    M::Result: Send + std::fmt::Debug,
    T: Serialize + Send + 'static,
    // E: std::fmt::Debug + Send + 'static,
    DbActor: Handler<M>
{
    match db.send(msg).await {
        Ok(Ok(res)) => HttpResponse::Ok().json(res),
        Ok(Err(err)) => match err {
            NotFound => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().body(format!("{}: {:?}", LEXICON["db_error"], err)),
        },
        Err(err) => HttpResponse::InternalServerError().body(format!("{}: {}", LEXICON["mailbox_error"], err)),
    }
}

pub async fn get_claims(req: HttpRequest, token: &str) -> Result<Claims, APIError> {
    if let Some(cookie) = req.cookie(token) {
        match verify_jwt(cookie.value()) {
            Err(_) => Err(APIError::InvalidToken),
            Ok(data) => Ok(data.claims)
        }
    }else{
        Err(APIError::MissingToken)
    }
}
