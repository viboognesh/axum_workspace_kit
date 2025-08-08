use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError, ValidationErrors};

use crate::constants::permissions;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleWithPermissions {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleResponse {
    pub status: &'static str,
    pub roles: Vec<RoleWithPermissions>,
}

pub trait RoleValidation {
    fn _name(&self) -> &str;
    fn _description(&self) -> &Option<String>;
    fn permissions(&self) -> &Vec<String>;

    fn validate_permissions(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        for permissions in self.permissions() {
            if !permissions::ALL.contains(&permissions.as_str()) {
                let mut error = ValidationError::new("Invalid");
                error.message = Some(format!("Invalid permissions:{}", permissions).into());
                errors.add("permissions", error);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn validate_all(&self) -> Result<(), ValidationErrors>
    where
        Self: Validate,
    {
        self.validate()?;
        self.validate_permissions()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateRoleDto {
    #[validate(length(min = 3, message = "Name must be at least 3 characters long"))]
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<String>,
}

impl RoleValidation for CreateRoleDto {
    fn _name(&self) -> &str {
        &self.name
    }
    fn _description(&self) -> &Option<String> {
        &self.description
    }
    fn permissions(&self) -> &Vec<String> {
        &self.permissions
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateRoleDto {
    #[validate(length(min = 3, message = "Name must be at least 3 characters long"))]
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<String>,
}

impl RoleValidation for UpdateRoleDto {
    fn _name(&self) -> &str {
        &self.name
    }
    fn _description(&self) -> &Option<String> {
        &self.description
    }
    fn permissions(&self) -> &Vec<String> {
        &self.permissions
    }
}
