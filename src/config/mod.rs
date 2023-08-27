pub struct Config {
    pub host: String,
    pub port: u16,
    pub workers: u16,
    pub secret_key: String,
}

pub enum Env {
    Development,
    Production,
}

impl Config {
    fn development() -> Config {
        Config {
            host: "127.0.0.1".to_string(),
            port: 8080,
            workers: 1,
            secret_key: "8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=".to_string(),
        }
    }

    fn production() -> Config {
        Config {
            host: std::env::var("HOST").expect("HOST not set"),
            port: std::env::var("PORT").expect("PORT not set").parse::<u16>().unwrap(),
            workers: std::env::var("WORKERS").unwrap_or("1".to_string()).parse::<u16>().unwrap(),
            secret_key: std::env::var("SECRET_KEY").expect("SECRET_KEY not set"),
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
