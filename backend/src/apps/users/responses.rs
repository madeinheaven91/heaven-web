use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct UserPublic {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub is_staff: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tokens {
    pub access_token: String,
    // pub refresh_token: String
}
