use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_maxage: i64,
    pub port: u16,
    pub backend_base_url: String,
    pub frontend_base_url: String,
}

impl Config {
    pub fn init() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in env");
        let jwt_secret = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY not set in env");
        let jwt_maxage = env::var("JWT_MAXAGE")
            .expect("JWT_MAXAGE not set in env")
            .parse()
            .expect("JWT MAXAGE must be a number");
        let port = env::var("PORT")
            .expect("PORT is not set in env")
            .parse()
            .expect("PORT must be a number");
        let backend_base_url =
            env::var("BACKEND_BASE_URL").expect("BACKEND_BASE_URL is not set in env");
        let frontend_base_url =
            env::var("FRONTEND_BASE_URL").expect("FRONTEND_BASE_URL is not set in env");

        Config {
            database_url: database_url,
            jwt_secret: jwt_secret,
            jwt_maxage: jwt_maxage,
            port: port,
            backend_base_url: backend_base_url,
            frontend_base_url: frontend_base_url,
        }
    }
}
