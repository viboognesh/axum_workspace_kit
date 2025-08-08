use async_trait::async_trait;
use sqlx::QueryBuilder;
use uuid::Uuid;

use crate::{
    database::DBClient,
    dtos::workspace::{
        FilterWorkspaceResponse, WorkspaceListDto, WorkspaceWithRoleAndPermissions,
        WorkspaceWithRoleAndPermissionsRow,
    },
    models::Workspace,
};

#[async_trait]
pub trait WorkspaceExt {
    async fn create_workspace(&self, name: &str, user_id: Uuid) -> Result<Workspace, sqlx::Error>;

    async fn get_workspace_details(
        &self,
        user_id: Option<Uuid>,
        workspace_id: Option<Uuid>,
    ) -> Result<WorkspaceWithRoleAndPermissions, sqlx::Error>;

    async fn update_workspace(&self, workspace_id: Uuid, name: &str) -> Result<(), sqlx::Error>;

    async fn delete_workspace(&self, workspace_id: Uuid) -> Result<(), sqlx::Error>;

    async fn get_all_user_workspace(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<WorkspaceListDto>, sqlx::Error>;
}

#[async_trait]
impl WorkspaceExt for DBClient {
    async fn create_workspace(&self, name: &str, user_id: Uuid) -> Result<Workspace, sqlx::Error> {
        sqlx::query_as!(
            Workspace,
            r#"
            INSERT INTO workspaces (name, owner_user_id, is_default)
            VALUES ($1, $2, true)
            RETURNING *
        "#,
            name,
            user_id
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn get_workspace_details(
        &self,
        user_id: Option<Uuid>,
        workspace_id: Option<Uuid>,
    ) -> Result<WorkspaceWithRoleAndPermissions, sqlx::Error> {
        let mut builder = QueryBuilder::new(
            r#"
            SELECT w.id as "workspace_id",
                w.name as "workspace_name",
                w.owner_user_id as "owner_user_id",
                w.is_default as "workspace_is_default",
                w.invite_code as "invite_code",
                w.created_at as "created_at",
                w.updated_at as "updated_at",
                r.id as "role_id",
                r.name as "role_name",
                COALESCE(
                    jsonb_agg(p.name) FILTER (
                        WHERE p.id IS NOT NULL
                    ),
                    '[]'::jsonb
                )::jsonb as "permissions"
            FROM workspaces w
                JOIN workspace_users wu ON w.id = wu.workspace_id
                JOIN roles r ON wu.role_id = r.id
                LEFT JOIN role_permissions rp ON r.id = rp.role_id
                LEFT JOIN permissions p ON rp.permission_id = p.id
            WHERE
            "#,
        );

        if let Some(workspace_id) = workspace_id {
            builder
                .push(" w.id = ")
                .push_bind(workspace_id)
                .push(" AND wu.user_id = ")
                .push_bind(user_id.unwrap());
        } else if let Some(user_id) = user_id {
            builder
                .push(" wu.user_id = ")
                .push_bind(user_id)
                .push(" AND w.is_default = true ");
        }

        builder.push(
            r#"
            GROUP BY w.id, w.name, w.owner_user_id, w.is_default, w.invite_code, w.created_at, w.updated_at, r.id, r.name
            "#
        );

        let query = builder.build_query_as::<WorkspaceWithRoleAndPermissionsRow>();

        let row = query.fetch_one(&self.pool).await?;

        let permissions: Vec<String> =
            serde_json::from_value(row.permissions).unwrap_or_else(|_| vec![]);

        Ok(WorkspaceWithRoleAndPermissions {
            workspace: FilterWorkspaceResponse {
                id: row.workspace_id,
                name: row.workspace_name,
                owner_user_id: row.owner_user_id.unwrap_or_default(),
                is_default: row.workspace_is_default,
                invite_code: row.invite_code,
                created_at: row.created_at,
                updated_at: row.updated_at,
            },
            role_id: row.role_id,
            role_name: row.role_name,
            permissions: permissions,
        })
    }

    async fn update_workspace(&self, workspace_id: Uuid, name: &str) -> Result<(), sqlx::Error> {
        let query = sqlx::query!(
            r#"
            UPDATE workspaces
            SET name = $1
            WHERE id = $2
            "#,
            name,
            workspace_id
        );

        query.execute(&self.pool).await?;

        Ok(())
    }

    async fn delete_workspace(&self, workspace_id: Uuid) -> Result<(), sqlx::Error> {
        let query = sqlx::query!(
            r#"
            DELETE FROM workspaces
            WHERE id = $1
            "#,
            workspace_id
        );

        query.execute(&self.pool).await?;

        Ok(())
    }

    async fn get_all_user_workspace(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<WorkspaceListDto>, sqlx::Error> {
        let workspace = sqlx::query_as!(
            WorkspaceListDto,
            r#"
                SELECT 
                    w.id as workspace_id,
                    w.name as workspace_name,
                    w.is_default,
                    w.invite_code,
                    w.created_at,
                    w.updated_at,
                    r.name as role_name
                FROM workspaces w
                    JOIN workspace_users wu ON w.id = wu.workspace_id
                    LEFT JOIN roles r ON wu.role_id = r.id
                WHERE wu.user_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(workspace)
    }
}
