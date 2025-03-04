use serde::Serialize;

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct UserPublic {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub is_staff: bool
}

#[derive(Serialize, Debug, Clone)]
pub struct Tokens {
    pub access: String,
    pub refresh: String
}
