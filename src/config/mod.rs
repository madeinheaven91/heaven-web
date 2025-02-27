use std::env;

#[derive(Debug)]
pub struct Config {
    pub server: ServerConfig,
    // pub db: tokio_postgres::Config,
}

#[derive(Debug)]
pub struct ServerConfig {
    pub environment: Environment,
    pub ip: String,
    pub port: u16,
}

impl Config {
    pub fn init(name: &str) -> Self {
        let _ = dotenv::from_filename(name);
        // let mut db = tokio_postgres::Config::new();
        // db.host(env::var("DB_HOST").unwrap_or(String::from("localhost")))
        //     .port(
        //         env::var("DB_PORT")
        //             .unwrap_or(String::from("5432"))
        //             .parse()
        //             .unwrap_or(5432),
        //     )
        //     .dbname(env::var("DB_NAME").unwrap_or(String::from("postgres")))
        //     .user(env::var("DB_USER").unwrap_or(String::from("admin")))
        //     .password(env::var("DB_PASS").unwrap_or(String::from("123")));
        Config {
            server: ServerConfig {
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
            },
            // db,
        }
    }

    // TODO:
}

#[derive(Debug)]
pub enum Environment {
    Development,
    Production,
}
