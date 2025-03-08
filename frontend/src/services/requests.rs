use reqwest::{ClientBuilder, Client};

use crate::shared::{statics::CONFIG, utils::get_access_token};
// TODO: error handling
pub async fn publish_post(slug: &str) {
    let client = ClientBuilder::new()
        .build()
        .unwrap();
    let req = client.post(format!("{}/blog/posts/{slug}", CONFIG.api_url()))
            .bearer_auth(get_access_token().unwrap_or_default())
            .header("Content-Type", "application/json")
            .body("{{\"is_published\": true}}");
    let resp = req.send()
        .await
        .unwrap();
}

pub async fn delete_post(slug: &str) -> Result<(), ()>{
    let client = ClientBuilder::new()
        .build()
        .unwrap();
    let req = client.delete(format!("{}/blog/posts/{slug}", CONFIG.api_url()))
            .bearer_auth(get_access_token().unwrap_or_default());
    let resp = req.send()
        .await
        .unwrap();
    Ok(())
}
