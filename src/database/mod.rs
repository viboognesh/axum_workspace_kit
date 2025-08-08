use sqlx::{Pool, Postgres};

pub mod auth;
pub mod permissions;
pub mod role;
pub mod user;
pub mod workspace;
pub mod workspace_user;

#[derive(Debug, Clone)]
pub struct DBClient {
    pool: Pool<Postgres>,
}

impl DBClient {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}
