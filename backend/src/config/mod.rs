use std::env;

use chrono::Local;
use log::LevelFilter;

#[derive(Debug)]
pub struct Config {
    pub environment: Environment,
    pub ip: String,
    pub port: u16,
    pub secret_key: String,
    pub db_url: String,
    pub log_level: LevelFilter,
    pub log_file: String,
}

impl Config {
    pub fn init() -> Self {
        Config {
            environment: {
                match env::var("ENVIRONMENT").unwrap_or_default().as_str() {
                    "prod" => Environment::Production,
                    _ => Environment::Development {
                        auth_enabled: env::var("AUTH_ENABLED")
                            .unwrap_or(String::from("true"))
                            .parse()
                            .unwrap_or(false),
                    },
                }
            },
            ip: env::var("SERVER_IP").unwrap_or(String::from("0.0.0.0")),
            port: env::var("SERVER_PORT")
                .unwrap_or(String::from("8000"))
                .parse()
                .unwrap_or(8000),
            secret_key: env::var("SECRET_KEY")
                .unwrap_or_else(|_| panic!("Secret key not set in env")),
            db_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| panic!("Database url not set in env")),
            log_level: env::var("RUST_LOG").unwrap_or("info".to_string()).parse().unwrap_or(LevelFilter::Info),
            log_file: env::var("LOG_FILE")
                .unwrap_or_else(|_| Local::now().format("%Y-%m-%d").to_string()),
        }
    }
}

#[derive(Debug)]
pub enum Environment {
    Development { auth_enabled: bool },
    Production,
}

impl Environment {
    pub fn auth_enabled(&self) -> bool {
        match self {
            Environment::Production => true,
            Environment::Development { auth_enabled, .. } => *auth_enabled,
        }
    }
}
