use actix_web::{
    body::MessageBody, dev::{ServiceRequest, ServiceResponse}, http::header, middleware::Next, Error, HttpMessage
};

use crate::shared::utils::verify_jwt;

pub async fn access_mw(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // Check for Authorization header
    if let Some(auth_header) = req.headers().get(header::AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                // Validate the token
                match verify_jwt(token) {
                    Ok(token_data) => {
                        // Token is valid, attach user ID to request extensions
                        req.extensions_mut().insert(token_data.claims);
                        return next.call(req).await;
                    } Err(_) => {
                        // Invalid token
                        return Err(actix_web::error::ErrorUnauthorized("Invalid access token"));
                    }
                }
            }
        }
    };

    // No or malformed Authorization header
    Err(actix_web::error::ErrorUnauthorized("Missing access token"))
}

pub async fn refresh_mw(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // Cookies
    if let Some(cookie) = req.cookie("refresh_token") {
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
                return Err(actix_web::error::ErrorUnauthorized("Invalid refresh token"));
            }
        }
    };
    // No cookie
    Err(actix_web::error::ErrorUnauthorized(
        "Missing refresh token",
    ))

}

pub async fn admin_access_mw(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // Check for Authorization header
    if let Some(auth_header) = req.headers().get(header::AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                // Validate the token
                match verify_jwt(token) {
                    Ok(token_data) => {
                        // Token is valid, attach user ID to request extensions
                        if !token_data.claims.staff {
                            return Err(actix_web::error::ErrorForbidden("Forbidden"));
                        }
                        req.extensions_mut().insert(token_data.claims);
                        return next.call(req).await;
                    } Err(_) => {
                        // Invalid token
                        return Err(actix_web::error::ErrorUnauthorized("Invalid access token"));
                    }
                }
            }
        }
    };

    // No or malformed Authorization header
    Err(actix_web::error::ErrorUnauthorized("Missing access token"))
}
