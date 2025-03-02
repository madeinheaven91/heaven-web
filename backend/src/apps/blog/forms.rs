use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct PostCreateForm {
    pub title: String,
    pub body: String,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct PostUpdateForm {
    pub title: Option<String>,
    pub body: Option<String>,
    pub is_published: Option<bool>
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct TagCreateForm {
    pub name: String,
    pub background_color: Option<String>,
    pub foreground_color: Option<String>
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct TagUpdateForm {
    pub name: Option<String>,
    pub background_color: Option<String>,
    pub foreground_color: Option<String>
}
