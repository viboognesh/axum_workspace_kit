use async_trait::async_trait;
use uuid::Uuid;

use crate::{database::DBClient, dtos::workspace_user::WorkspaceUserWithRoleDto};

#[async_trait]
pub trait WorkspaceUserExt {
    async fn join_workspace(&self, user_id: Uuid, invite_code: &str) -> Result<Uuid, sqlx::Error>;

    async fn remove_workspace(&self, user_id: Uuid, workspace_id: Uuid) -> Result<(), sqlx::Error>;

    async fn get_workspace_users(
        &self,
        workspace_id: Uuid,
    ) -> Result<Vec<WorkspaceUserWithRoleDto>, sqlx::Error>;

    async fn update_workspace_user_role(
        &self,
        workspace_id: Uuid,
        user_id: Uuid,
        role_id: Uuid,
    ) -> Result<(), sqlx::Error>;
}

#[async_trait]
impl WorkspaceUserExt for DBClient {
    async fn join_workspace(&self, user_id: Uuid, invite_code: &str) -> Result<Uuid, sqlx::Error> {
        let record = sqlx::query!(
            r#"
                WITH selected_workspace AS (
                    SELECT w.id as workspace_id,
                        r.id as role_id
                    FROM workspaces w
                        JOIN roles r ON r.workspace_id = w.id
                    WHERE invite_code = $1
                        AND r.name != 'Admin'
                    ORDER BY r.name ASC
                    LIMIT 1
                )
                INSERT INTO workspace_users (workspace_id, user_id, role_id)
                SELECT workspace_id,
                    $2 as user_id,
                    role_id
                FROM selected_workspace ON CONFLICT (workspace_id, user_id) DO NOTHING
                RETURNING workspace_id
            "#,
            invite_code,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        let workspace_id = record
            .map(|row| row.workspace_id)
            .ok_or(sqlx::Error::RowNotFound)?;

        Ok(workspace_id)
    }

    async fn remove_workspace(&self, user_id: Uuid, workspace_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM workspace_users
            WHERE user_id = $1 AND workspace_id = $2
            "#,
            user_id,
            workspace_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_workspace_users(
        &self,
        workspace_id: Uuid,
    ) -> Result<Vec<WorkspaceUserWithRoleDto>, sqlx::Error> {
        sqlx::query_as!(
            WorkspaceUserWithRoleDto,
            r#"
                SELECT u.id as user_id,
                    u.name as user_name,
                    u.email as user_email,
                    r.name as role_name
                FROM workspace_users wu
                    JOIN users u ON wu.user_id = u.id
                    JOIN roles r ON wu.role_id = r.id
                WHERE wu.workspace_id = $1
            "#,
            workspace_id
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn update_workspace_user_role(
        &self,
        workspace_id: Uuid,
        user_id: Uuid,
        role_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE workspace_users
            SET role_id = $1
            WHERE user_id = $2 AND workspace_id = $3
            "#,
            role_id,
            user_id,
            workspace_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
