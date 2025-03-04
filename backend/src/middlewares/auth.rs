use actix_web::{
    Error, HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
};

use crate::shared::utils::verify_jwt;

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // Cookies
    if let Some(cookie) = req.cookie("access_token") {
        let token = cookie.value();
        // Validate the token
        match verify_jwt(token) {
            Ok(token_data) => {
                // Token is valid, attach user claims to request extensions
                req.extensions_mut().insert(token_data.claims);
                return next.call(req).await;
            }
            Err(_) => {
                // Invalid token
                return Err(actix_web::error::ErrorUnauthorized("Invalid token"));
            }
        }
    };
    // No cookie
    Err(actix_web::error::ErrorUnauthorized(
        "Missing token",
    ))
}
