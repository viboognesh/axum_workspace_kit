use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::Path,
    response::IntoResponse,
    routing::{get, post},
};
use uuid::Uuid;

use crate::{
    AppState,
    constants::permissions,
    database::role::RoleExt,
    dtos::{
        Response,
        role::{CreateRoleDto, RoleResponse, RoleValidation, UpdateRoleDto},
    },
    error::HttpError,
    middleware::{
        jwt_auth_middleware::JwtAuthMiddleware, workspace_middleware::WorkspaceAuthMiddleware,
    },
    workspace_auth,
};

pub fn role_handler() -> axum::Router {
    axum::Router::new()
        .route(
            "/",
            get(get_roles).layer(workspace_auth!(permissions::VIEW_ROLES)),
        )
        .route(
            "/",
            post(create_roles).layer(workspace_auth!(permissions::MANAGE_ROLES)),
        )
        .route(
            "/{role_id}",
            axum::routing::put(update_role)
                .delete(delete_role)
                .layer(workspace_auth!(permissions::MANAGE_ROLES)),
        )
}

pub async fn get_roles(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(_user): Extension<JwtAuthMiddleware>,
    Extension(workspace): Extension<WorkspaceAuthMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
    let roles = app_state
        .db_client
        .get_role_with_permissions(workspace.workspace_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = RoleResponse {
        status: "success",
        roles,
    };

    Ok(Json(response))
}

pub async fn create_roles(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(_user): Extension<JwtAuthMiddleware>,
    Extension(workspace): Extension<WorkspaceAuthMiddleware>,
    Json(payload): Json<CreateRoleDto>,
) -> Result<impl IntoResponse, HttpError> {
    payload
        .validate_all()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    app_state
        .db_client
        .create_role(
            workspace.workspace_id,
            payload.name,
            payload.description.as_deref(),
            payload.permissions,
        )
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = Response {
        status: "success",
        message: "Role created successfully".to_string(),
    };

    Ok(Json(response))
}

pub async fn update_role(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(_user): Extension<JwtAuthMiddleware>,
    Extension(workspace): Extension<WorkspaceAuthMiddleware>,
    Path(role_id): Path<Uuid>,
    Json(payload): Json<UpdateRoleDto>,
) -> Result<impl IntoResponse, HttpError> {
    payload
        .validate_all()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    app_state
        .db_client
        .update_role(
            workspace.workspace_id,
            role_id,
            payload.name,
            payload.description.as_deref(),
            payload.permissions,
        )
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = Response {
        status: "success",
        message: "Role updated successfully".to_string(),
    };

    Ok(Json(response))
}

pub async fn delete_role(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(_user): Extension<JwtAuthMiddleware>,
    Extension(workspace): Extension<WorkspaceAuthMiddleware>,
    Path(role_id): Path<Uuid>,
) -> Result<impl IntoResponse, HttpError> {
    app_state
        .db_client
        .delete_role(workspace.workspace_id, role_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = Response {
        status: "success",
        message: "Role deleted successfully".to_string(),
    };

    Ok(Json(response))
}
