use crate::{database::DBClient, dtos::permissions::PermissionDto};
use async_trait::async_trait;
use sqlx::Error;

#[async_trait]
pub trait PermissionExt {
    async fn get_permissions(&self) -> Result<Vec<PermissionDto>, Error>;
}

#[async_trait]
impl PermissionExt for DBClient {
    async fn get_permissions(&self) -> Result<Vec<PermissionDto>, Error> {
        let permissions = sqlx::query_as!(
            PermissionDto,
            r#"
            SELECT name, description 
            FROM permissions
            "#,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(permissions)
    }
}
