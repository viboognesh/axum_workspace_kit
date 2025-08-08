use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{dtos::auth::validate_password_complexity, models::User};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterUserDto {
    pub id: Option<Uuid>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub verified: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl FilterUserDto {
    pub fn filter_user(user: &User) -> Self {
        Self {
            id: Some(user.id),
            email: Some(user.email.clone()),
            name: Some(user.name.clone()),
            verified: user.email_verified.unwrap_or(false),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
    pub user: FilterUserDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub status: &'static str,
    pub data: UserData,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UserPasswordUpdate {
    #[validate(
        length(
            min = 8,
            max = 64,
            message = "Password must be between 8 and 64 characters"
        ),
        custom(function = "validate_password_complexity",)
    )]
    #[serde(rename = "currentPassword")]
    pub current_password: String,

    #[validate(
        length(
            min = 8,
            max = 64,
            message = "Password must be between 8 and 64 characters"
        ),
        custom(function = "validate_password_complexity",)
    )]
    #[serde(rename = "newPassword")]
    pub new_password: String,

    #[validate(
        length(
            min = 8,
            max = 64,
            message = "Password must be between 8 and 64 characters"
        ),
        must_match(other = "new_password", message = "Passwords do not match"),
        custom(function = "validate_password_complexity",)
    )]
    #[serde(rename = "confirmPassword")]
    pub confirm_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UserEmailChangeRequest {
    #[validate(
        length(
            min = 5,
            max = 254,
            message = "Email must be between 5 and 254 characters"
        ),
        email(message = "Invalid email address")
    )]
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UserEmailChangeVerificationDto {
    #[validate(length(min = 1, message = "Token is required"))]
    pub token: String,
}
