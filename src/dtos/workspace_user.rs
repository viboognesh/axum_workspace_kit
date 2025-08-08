use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WorkspaceUserWithRoleDto {
    pub user_id: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub role_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSpaceUsers {
    pub users: Vec<WorkspaceUserWithRoleDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSpaceUserResponseWithRoleDto {
    pub status: &'static str,
    pub data: WorkSpaceUsers,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserRoleDto {
    #[validate(length(min = 1, message = "Role name is required"))]
    pub role_name: String,
}
