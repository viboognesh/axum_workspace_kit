use std::sync::Arc;

use axum::{Extension, Json, response::IntoResponse};

use crate::{
    AppState,
    constants::permissions,
    database::permissions::PermissionExt,
    dtos::permissions::{PermissionDtoResponse, PermissionsDto},
    error::HttpError,
    middleware::{
        jwt_auth_middleware::JwtAuthMiddleware, workspace_middleware::WorkspaceAuthMiddleware,
    },
    workspace_auth,
};

pub fn permissions_handler() -> axum::Router {
    axum::Router::new().route(
        "/",
        axum::routing::get(get_permissions).layer(workspace_auth!(permissions::VIEW_PERMISSIONS)),
    )
}

pub async fn get_permissions(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(_user): Extension<JwtAuthMiddleware>,
    Extension(_workspace): Extension<WorkspaceAuthMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
    let permissions = app_state
        .db_client
        .get_permissions()
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = PermissionDtoResponse {
        status: "success",
        data: PermissionsDto { permissions },
    };

    Ok(Json(response))
}
