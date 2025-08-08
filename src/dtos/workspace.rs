use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WorkspaceCreateDto {
    #[validate(length(min = 1, message = "Workspace name is required"))]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterWorkspaceResponse {
    pub id: Uuid,
    pub name: String,
    pub owner_user_id: Uuid,
    pub invite_code: String,
    pub is_default: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceCreateResponseDto {
    pub status: &'static str,
    pub data: WorkspaceWithRoleAndPermissions,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WorkspaceWithRoleAndPermissionsRow {
    pub workspace_id: Uuid,
    pub workspace_name: String,
    pub owner_user_id: Option<Uuid>,
    pub invite_code: String,
    pub workspace_is_default: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub role_id: Uuid,
    pub role_name: String,
    pub permissions: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceDetailsResponseDto {
    pub status: &'static str,
    pub data: WorkspaceWithRoleAndPermissions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceWithRoleAndPermissions {
    pub workspace: FilterWorkspaceResponse,
    pub role_id: Uuid,
    pub role_name: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateWorkspaceDto {
    #[validate(length(min = 3, message = "Workspace name must be at least 3 characters long"))]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WorkspaceListDto {
    pub workspace_id: Uuid,
    pub workspace_name: String,
    pub invite_code: String,
    pub is_default: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub role_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceListResponse {
    pub status: &'static str,
    pub data: WorkspaceList,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceList {
    pub workspaces: Vec<WorkspaceListDto>,
}
