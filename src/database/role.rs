use async_trait::async_trait;
use uuid::Uuid;

use crate::{database::DBClient, dtos::role::RoleWithPermissions};

#[async_trait]
pub trait RoleExt {
    async fn get_role_with_permissions(
        &self,
        workspace_id: Uuid,
    ) -> Result<Vec<RoleWithPermissions>, sqlx::Error>;

    async fn create_role(
        &self,
        workspace_id: Uuid,
        name: String,
        description: Option<&str>,
        permissions: Vec<String>,
    ) -> Result<(), sqlx::Error>;

    async fn delete_role(&self, workspace_id: Uuid, role_id: Uuid) -> Result<(), sqlx::Error>;

    async fn update_role(
        &self,
        workspace_id: Uuid,
        role_id: Uuid,
        name: String,
        description: Option<&str>,
        permissions: Vec<String>,
    ) -> Result<(), sqlx::Error>;

    async fn get_role_id_by_name(
        &self,
        workspace_id: Uuid,
        name: String,
    ) -> Result<Uuid, sqlx::Error>;
}

#[async_trait]
impl RoleExt for DBClient {
    async fn get_role_with_permissions(
        &self,
        workspace_id: Uuid,
    ) -> Result<Vec<RoleWithPermissions>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                r.id as id,
                r.name as name,
                r.description as description,
                ARRAY_REMOVE(ARRAY_AGG(p.name), NULL) as permissions
            FROM roles r
            LEFT JOIN role_permissions rp ON r.id = rp.role_id
            LEFT JOIN permissions p ON rp.permission_id = p.id
            WHERE r.workspace_id = $1 and r.name != 'Admin'
            GROUP BY r.id, r.name, r.description
            "#,
            workspace_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|row| RoleWithPermissions {
                id: row.id,
                name: row.name,
                description: row.description,
                permissions: row.permissions.unwrap_or_default(),
            })
            .collect())
    }

    async fn create_role(
        &self,
        workspace_id: Uuid,
        name: String,
        description: Option<&str>,
        permissions: Vec<String>,
    ) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        let role = sqlx::query!(
            r#"
            INSERT INTO roles (workspace_id, name, description) 
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
            workspace_id,
            name,
            description
        )
        .fetch_one(&mut *tx)
        .await?;

        for permission in permissions {
            sqlx::query!(
                r#"
                INSERT INTO role_permissions (role_id, permission_id)
                SELECT $1, id FROM permissions WHERE name = $2
                "#,
                role.id,
                permission
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }

    async fn delete_role(&self, workspace_id: Uuid, role_id: Uuid) -> Result<(), sqlx::Error> {
        let is_admin = sqlx::query_scalar!(
            r#"
            SELECT EXISTS (SELECT 1 FROM roles WHERE id = $1 AND workspace_id = $2 AND name = 'Admin')
            "#,
            role_id,
            workspace_id,
        ).fetch_one(&self.pool).await?;

        if is_admin.unwrap_or(false) {
            return Err(sqlx::Error::RowNotFound);
        }

        sqlx::query!(
            r#"
            DELETE FROM roles
            WHERE id = $1 AND workspace_id = $2
            "#,
            role_id,
            workspace_id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_role(
        &self,
        workspace_id: Uuid,
        role_id: Uuid,
        name: String,
        description: Option<&str>,
        permissions: Vec<String>,
    ) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        let is_admin = sqlx::query_scalar!(
            r#"
            SELECT EXISTS (SELECT 1 FROM roles WHERE id = $1 AND workspace_id = $2 AND name = 'Admin')
            "#,
            role_id,
            workspace_id,
        ).fetch_one(&self.pool).await?;

        if is_admin.unwrap_or(false) {
            return Err(sqlx::Error::RowNotFound);
        }

        sqlx::query!(
            r#"
            UPDATE roles
            SET name = $1, description = $2
            WHERE id = $3 AND workspace_id = $4
            "#,
            name,
            description,
            role_id,
            workspace_id
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            r#"
            DELETE FROM role_permissions
            WHERE role_id = $1
            "#,
            role_id
        )
        .execute(&mut *tx)
        .await?;

        for permission_name in permissions {
            sqlx::query!(
                r#"
                INSERT INTO role_permissions (role_id, permission_id)
                SELECT $1, id FROM permissions WHERE name = $2
                "#,
                role_id,
                permission_name
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }

    async fn get_role_id_by_name(
        &self,
        workspace_id: Uuid,
        name: String,
    ) -> Result<Uuid, sqlx::Error> {
        let role_id = sqlx::query_scalar!(
            r#"
            SELECT id FROM roles WHERE workspace_id = $1 and name = $2
            "#,
            workspace_id,
            name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(role_id)
    }
}
