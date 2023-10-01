pub struct Config {
    pub host: String,
    pub port: u16,
    pub postgres_url: String,
    pub nats_url: String,
    pub jwt_secret: String,
}
use lazy_static::lazy_static;

pub enum Env {
    Development,
    Production,
}

impl Config {
    fn development() -> Config {
        Config {
            host: "127.0.0.1".to_string(),
            port: 8080,
            postgres_url: std::env::var("POSTGRES_URL")
                .unwrap_or("postgres://postgres:changeme@localhost:5432".to_string()),
            nats_url: std::env::var("NATS_URL").unwrap_or("localhost:4222".to_string()),
            jwt_secret: "very-very-secure-secrete".to_string(),
        }
    }

    fn production() -> Config {
        Config {
            host: std::env::var("HOST").expect("HOST not set"),
            port: std::env::var("PORT")
                .expect("PORT not set")
                .parse::<u16>()
                .expect("PORT must be an integer"),
            postgres_url: std::env::var("POSTGRES_URL").expect("POSTGRES_URL not set"),
            nats_url: std::env::var("NATS_URL").expect("NATS_URL not set"),
            jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET not set"),
        }
    }

    pub fn get() -> Config {
        match Config::running_env() {
            Env::Development => Config::development(),
            Env::Production => Config::production(),
        }
    }

    pub fn running_env() -> Env {
        match std::env::var("ENV") {
            Ok(val) => match val.as_str() {
                "dev" | "development" => Env::Development,
                "prod" | "production" => Env::Production,
                _ => Env::Development,
            },
            Err(_) => Env::Development,
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::get();
}
