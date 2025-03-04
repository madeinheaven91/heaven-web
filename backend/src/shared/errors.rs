use std::{error::Error, fmt::Display};

use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

use super::statics::LEXICON;

#[derive(Debug, Serialize, Deserialize)]
pub enum APIError {
    MissingToken,
    InvalidToken,
    InvalidInput(String),
    MailboxError,
    Forbidden
}

impl APIError {
    pub fn to_http(&self) -> HttpResponse{
        match self {
            Self::MissingToken => HttpResponse::Unauthorized().content_type("text/html").body(self.to_string()),
            Self::InvalidToken => HttpResponse::Unauthorized().content_type("text/html").body(self.to_string()),
            Self::MailboxError => HttpResponse::InternalServerError().content_type("text/html").body(LEXICON["mailbox_error"]),
            Self::Forbidden => HttpResponse::Forbidden().finish(),
            _ => unimplemented!()
        }
    }
}

impl Display for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Self::MissingToken => "Missing token".to_string(),
            Self::InvalidToken => "Invalid token".to_string(),
            Self::InvalidInput(input) => format!("Invalid input: {}", input),
            Self::MailboxError => "MailboxError".to_string(),
            Self::Forbidden => "Forbidden".to_string(),
        };
        write!(f, "{}", res)?;
        Ok(())
    }
}

impl Error for APIError {}
