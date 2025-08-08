use serde::{Deserialize, Serialize};

pub mod auth;
pub mod permissions;
pub mod role;
pub mod user;
pub mod workspace;
pub mod workspace_user;

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}
