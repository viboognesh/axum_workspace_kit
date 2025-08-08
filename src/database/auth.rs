use crate::{
    database::DBClient,
    models::{EmailVerification, PasswordReset, User},
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{self, Error};
use uuid::Uuid;

#[async_trait]
pub trait AuthExt {
    async fn get_user(
        &self,
        user_id: Option<Uuid>,
        name: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<User>, Error>;

    async fn save_user<T: Into<String> + Send>(
        &self,
        name: T,
        email: T,
        password: T,
        verfification_token: Uuid,
        token_expires_at: DateTime<Utc>,
    ) -> Result<User, Error>;

    async fn get_user_id_by_token(&self, token: Uuid) -> Result<Option<EmailVerification>, Error>;

    async fn verify_user(&self, user_id: Uuid) -> Result<(), Error>;

    async fn save_password_reset_token(
        &self,
        user_id: Uuid,
        token: Uuid,
        expires_at: DateTime<Utc>,
    ) -> Result<(), Error>;

    async fn get_password_reset_token(&self, token: Uuid) -> Result<Option<PasswordReset>, Error>;

    async fn reset_password(&self, user_id: Uuid, new_password: &str) -> Result<(), Error>;

    async fn update_user_password(&self, user_id: Uuid, new_password: &str) -> Result<(), Error>;
}

#[async_trait]
impl AuthExt for DBClient {
    async fn get_user(
        &self,
        user_id: Option<Uuid>,
        name: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<User>, Error> {
        let mut query = String::from("SELECT * FROM users WHERE 1 = 1");
        if let Some(_id) = user_id {
            query.push_str(" AND id = $1");
        }
        if let Some(_n) = name {
            query.push_str("AND name = $2");
        }
        if let Some(_e) = email {
            query.push_str("AND email = $3");
        }

        let rows = sqlx::query_as::<_, User>(&query)
            .bind(user_id)
            .bind(name)
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;
        Ok(rows)
    }

    async fn save_user<T: Into<String> + Send>(
        &self,
        name: T,
        email: T,
        password: T,
        verification_token: Uuid,
        token_expires_at: DateTime<Utc>,
    ) -> Result<User, Error> {
        let name = name.into();
        let email = email.into();
        let password = password.into();

        let mut tx = self.pool.begin().await?;

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (name, email, password) 
            VALUES ($1, $2, $3)
            RETURNING id, name, email, password, pending_email, email_verified, pending_email_expires_at, pending_email_token, created_at, updated_at
            "#,
            name,
            email,
            password
        )
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query!(
            r#"
            INSERT INTO email_verifications (user_id, token, expires_at)
            VALUES ($1, $2::UUID, $3)
            "#,
            user.id,
            verification_token,
            token_expires_at,
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(user)
    }

    async fn get_user_id_by_token(&self, token: Uuid) -> Result<Option<EmailVerification>, Error> {
        let record = sqlx::query_as!(
            EmailVerification,
            r#"
            SELECT user_id, token, expires_at 
            FROM email_verifications 
            WHERE token = $1
            "#,
            token
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(record)
    }

    async fn verify_user(&self, user_id: Uuid) -> Result<(), Error> {
        sqlx::query!(
            r#"
            UPDATE users
            SET email_verified = true
            WHERE id = $1
            "#,
            user_id
        )
        .execute(&self.pool)
        .await?;

        sqlx::query!(
            r#"
            DELETE FROM email_verifications
            WHERE user_id = $1
            "#,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn save_password_reset_token(
        &self,
        user_id: Uuid,
        token: Uuid,
        expires_at: DateTime<Utc>,
    ) -> Result<(), Error> {
        sqlx::query!(
            r#"
            INSERT INTO password_resets (user_id, token, expires_at)
            VALUES ($1, $2::UUID, $3)
            "#,
            user_id,
            token,
            expires_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_password_reset_token(&self, token: Uuid) -> Result<Option<PasswordReset>, Error> {
        let record = sqlx::query_as!(
            PasswordReset,
            r#"
            SELECT user_id, token, expires_at 
            FROM password_resets 
            WHERE token = $1
            "#,
            token
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(record)
    }

    async fn reset_password(&self, user_id: Uuid, new_password: &str) -> Result<(), Error> {
        sqlx::query!(
            r#"
            UPDATE users
            SET password = $2
            WHERE id = $1
            "#,
            user_id,
            new_password
        )
        .execute(&self.pool)
        .await?;

        sqlx::query!(
            r#"
            DELETE FROM password_resets
            WHERE user_id = $1
            "#,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_user_password(&self, user_id: Uuid, new_password: &str) -> Result<(), Error> {
        sqlx::query!(
            r#"
            UPDATE users
            SET password = $2, updated_at = NOW()
            WHERE id = $1
            "#,
            user_id,
            new_password
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
