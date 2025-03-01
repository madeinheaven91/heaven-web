use std::env;

#[derive(Debug)]
pub struct Config {
    // pub server: ServerConfig,
    pub environment: Environment,
    pub ip: String,
    pub port: u16,
    pub secret_key: String,
    pub db_url: String
}

// #[derive(Debug)]
// pub struct ServerConfig {
//     pub environment: Environment,
//     pub ip: String,
//     pub port: u16,
//     pub secret_key: String,
//     pub db_url: String
// }

impl Config {
    pub fn init() -> Self {
        Config {
            // server: ServerConfig {
                environment: {
                    match env::var("ENVIRONMENT").unwrap_or_default().as_str() {
                        "prod" => Environment::Production,
                        _ => Environment::Development,
                    }
                },
                ip: env::var("SERVER_IP").unwrap_or(String::from("0.0.0.0")),
                port: env::var("SERVER_PORT")
                    .unwrap_or(String::from("8000"))
                    .parse()
                    .unwrap_or(8000),
                secret_key: env::var("SECRET_KEY").unwrap_or_else(|_| panic!("Secret key not set in env")),
                db_url: env::var("DATABASE_URL").unwrap_or_else(|_| panic!("Database url not set in env")),
            }
        // }
    }
}

#[derive(Debug)]
pub enum Environment {
    Development,
    Production,
}
