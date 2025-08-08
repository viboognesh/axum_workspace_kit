use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::dtos::{user::FilterUserDto, workspace::WorkspaceWithRoleAndPermissions};

pub fn validate_password_complexity(password: &str) -> Result<(), ValidationError> {
    let uppercase = Regex::new(r"[A-Z]").unwrap();
    let lowercase = Regex::new(r"[a-z]").unwrap();
    let number = Regex::new(r"[0-9]").unwrap();
    let special = Regex::new(r"[^A-Za-z0-9]").unwrap();
    if !uppercase.is_match(password) {
        return Err(ValidationError::new(
            "Password must contain at least one uppercase letter",
        ));
    }
    if !lowercase.is_match(password) {
        return Err(ValidationError::new(
            "Password must contain at least one lowercase letter",
        ));
    }
    if !number.is_match(password) {
        return Err(ValidationError::new(
            "Password must contain at least one number",
        ));
    }
    if !special.is_match(password) {
        return Err(ValidationError::new(
            "Password must contain at least one special character",
        ));
    }
    Ok(())
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct RegisterUserDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,

    #[validate(
        length(
            min = 5,
            max = 254,
            message = "Email must be between 5 and 254 characters"
        ),
        email(message = "Invalid email address")
    )]
    pub email: String,

    #[validate(
        length(
            min = 8,
            max = 64,
            message = "Password must be between 8 and 64 characters"
        ),
        custom(function = "validate_password_complexity",)
    )]
    pub password: String,

    #[validate(
        length(
            min = 8,
            max = 64,
            message = "Password must be between 8 and 64 characters"
        ),
        must_match(other = "password", message = "Passwords do not match")
    )]
    #[serde(rename = "passwordConfirm")]
    pub password_confirm: String,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct LoginUserDto {
    #[validate(
        length(
            min = 5,
            max = 254,
            message = "Email must be between 5 and 254 characters"
        ),
        email(message = "Invalid email address")
    )]
    pub email: String,

    #[validate(
        length(
            min = 8,
            max = 64,
            message = "Password must be between 8 and 64 characters"
        ),
        custom(function = "validate_password_complexity",)
    )]
    pub password: String,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct VerifyEmailQueryDto {
    #[validate(length(min = 1, message = "Token is required"))]
    pub token: String,
}

#[derive(Debug, Default, Clone, Validate, Serialize, Deserialize)]
pub struct ForgotPasswordDto {
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

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct ResetPasswordDto {
    #[validate(length(min = 1, message = "Token is required"))]
    pub token: String,

    #[validate(
        length(
            min = 8,
            max = 64,
            message = "Password must be between 8 and 64 characters"
        ),
        custom(function = "validate_password_complexity",)
    )]
    pub password: String,

    #[validate(
        length(
            min = 8,
            max = 64,
            message = "Password must be between 8 and 64 characters"
        ),
        must_match(other = "password", message = "Passwords do not match")
    )]
    #[serde(rename = "passwordConfirm")]
    pub password_confirm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub status: &'static str,
    pub token: String,
    pub data: UserDataResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDataResponse {
    pub user: FilterUserDto,
    pub workspace: Option<WorkspaceWithRoleAndPermissions>,
}
