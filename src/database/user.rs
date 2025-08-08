use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{database::DBClient, models::User};

#[async_trait]
pub trait UserExt {
    async fn update_user_email_request(
        &self,
        user_id: Uuid,
        email: String,
        token: Uuid,
        token_expires_at: DateTime<Utc>,
    ) -> Result<User, sqlx::Error>;

    async fn verify_email_change(&self, token: Uuid) -> Result<(), sqlx::Error>;
}

#[async_trait]
impl UserExt for DBClient {
    async fn update_user_email_request(
        &self,
        user_id: Uuid,
        email: String,
        token: Uuid,
        token_expires_at: DateTime<Utc>,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET pending_email = $1, pending_email_token = $2::UUID, pending_email_expires_at = $3
            WHERE id = $4
            RETURNING *
            "#,
            email,
            token,
            token_expires_at,
            user_id
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn verify_email_change(&self, token: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE users
            SET email = pending_email, pending_email = NULL, pending_email_token = NULL, pending_email_expires_at = NULL
            WHERE pending_email_token = $1
            "#,
            token
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
