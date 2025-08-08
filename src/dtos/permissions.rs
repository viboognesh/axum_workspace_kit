use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PermissionDto {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionDtoResponse {
    pub status: &'static str,
    pub data: PermissionsDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionsDto {
    pub permissions: Vec<PermissionDto>,
}
