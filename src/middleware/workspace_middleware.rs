use std::sync::Arc;

use axum::{Extension, extract::Request, middleware::Next, response::IntoResponse};
use axum_extra::extract::CookieJar;
use uuid::Uuid;

use crate::{
    AppState,
    database::workspace::WorkspaceExt,
    error::{ErrorMessage, HttpError},
    middleware::jwt_auth_middleware::JwtAuthMiddleware,
};

#[derive(Debug, Clone)]
pub struct RequirePermission(pub &'static str);

#[derive(Debug, Clone)]
pub struct WorkspaceAuthMiddleware {
    pub workspace_id: Uuid,
}

///
/// axum::middleware::from_fn(
/// |cookie_jar, extension, extension, req, next| async move {
/// workspace_permission_middleware(cookie_jar, extension, extension, req, next, RequirePermission("workspace:read")).await
/// }
/// )

pub async fn workspace_permission_middleware(
    cookie_jar: CookieJar,
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JwtAuthMiddleware>,
    mut req: Request,
    next: Next,
    permission: RequirePermission,
) -> Result<impl IntoResponse, HttpError> {
    let user_id = user.user.id;
    let workspace_cookie = cookie_jar
        .get("workspace")
        .map(|c| c.value().to_string())
        .ok_or(HttpError::unauthorized(
            "Workspace id not found".to_string(),
        ))?;

    let workspace_id = Uuid::parse_str(&workspace_cookie)
        .map_err(|_| HttpError::unauthorized("Invalid workspace id".to_string()))?;

    let workspace_details = app_state
        .db_client
        .get_workspace_details(Some(user_id), Some(workspace_id))
        .await
        .map_err(|_| HttpError::server_error(ErrorMessage::ServerError.to_string()))?;

    let has_permissions = workspace_details
        .permissions
        .iter()
        .any(|p| p == permission.0);

    if !has_permissions {
        return Err(HttpError::unauthorized(
            ErrorMessage::PermissionDenied.to_string(),
        ));
    }

    req.extensions_mut()
        .insert(WorkspaceAuthMiddleware { workspace_id });

    Ok(next.run(req).await)
}

#[macro_export]
macro_rules! workspace_auth {
    ($permission:expr) => {
        axum::middleware::from_fn(
            |cookie_jar, extension, extension_auth, req, next| async move {
                crate::middleware::workspace_middleware::workspace_permission_middleware(
                    cookie_jar,
                    extension,
                    extension_auth,
                    req,
                    next,
                    crate::middleware::workspace_middleware::RequirePermission($permission),
                )
                .await
            },
        )
    };
}
