use argon2::{password_hash::{self, rand_core::OsRng, SaltString}, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use super::statics::CONFIG;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims{
    pub sub: i32,
    pub exp: i64,
    pub staff: bool
}

pub fn create_jwt(sub: i32, staff: bool) -> String {
    let payload = Claims {
        sub,
        exp: (Utc::now() + Duration::hours(1)).timestamp(),
        staff
    };
    encode(&Header::new(jsonwebtoken::Algorithm::HS512), &payload, &EncodingKey::from_secret(CONFIG.secret_key.as_bytes())).unwrap()
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
