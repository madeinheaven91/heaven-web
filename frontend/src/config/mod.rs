use std::env;


#[derive(Debug)]
pub struct Config {
    pub frontend_port: u16,
    pub backend_port: u16
}

impl Config {
    pub fn init() -> Self {
        Config {
            backend_port: env::var("SERVER_PORT")
                .unwrap_or(String::from("8000"))
                .parse()
                .unwrap_or(8000),
            frontend_port: env::var("FRONTEND_PORT")
                .unwrap_or(String::from("3000"))
                .parse()
                .unwrap_or(3000),
        }
    }
}
