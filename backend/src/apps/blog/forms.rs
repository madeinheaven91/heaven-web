use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostUpdateForm {
    pub title: Option<String>,
    pub body: Option<String>,
    pub is_published: Option<bool>
}

#[derive(Deserialize)]
pub struct PostCreateForm {
    pub title: String,
    pub body: String,
}
