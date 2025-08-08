use std::sync::Arc;

use axum::{Extension, Json, extract::Path, response::IntoResponse};
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState,
    constants::permissions,
    database::{role::RoleExt, workspace::WorkspaceExt, workspace_user::WorkspaceUserExt},
    dtos::{
        Response,
        workspace_user::{UpdateUserRoleDto, WorkSpaceUserResponseWithRoleDto, WorkSpaceUsers},
    },
    error::HttpError,
    handlers::workspace::create_workspace_response,
    middleware::{
        jwt_auth_middleware::JwtAuthMiddleware, workspace_middleware::WorkspaceAuthMiddleware,
    },
    workspace_auth,
};

pub fn workspace_user_handler() -> axum::Router {
    axum::Router::new()
        .route("/invite/{invite_code}", axum::routing::get(join_workspace))
        .route(
            "/remove",
            axum::routing::delete(remove_user_from_workspace)
                .layer(workspace_auth!(permissions::REMOVE_MEMBERS)),
        )
        .route(
            "/",
            axum::routing::get(get_workspace_users)
                .layer(workspace_auth!(permissions::VIEW_MEMBERS)),
        )
        .route(
            "/{user_id}",
            axum::routing::patch(update_user_role)
                .layer(workspace_auth!(permissions::ASSIGN_ROLES_TO_MEMBERS)),
        )
}

pub async fn join_workspace(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JwtAuthMiddleware>,
    Path(invite_code): Path<String>,
) -> Result<impl IntoResponse, HttpError> {
    let workspace_id = app_state
        .db_client
        .join_workspace(user.user.id, &invite_code)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => HttpError::bad_request("Invalid invite code".to_string()),
            _ => HttpError::server_error(e.to_string()),
        })?;

    let workspace_with_role_and_permissions = app_state
        .db_client
        .get_workspace_details(Some(user.user.id), Some(workspace_id))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    create_workspace_response(workspace_with_role_and_permissions, app_state)
}

pub async fn remove_user_from_workspace(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(_user): Extension<JwtAuthMiddleware>,
    Extension(workspace): Extension<WorkspaceAuthMiddleware>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, HttpError> {
    app_state
        .db_client
        .remove_workspace(user_id, workspace.workspace_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = Response {
        status: "success",
        message: "Removed user from workspace successfully".to_string(),
    };

    Ok(Json(response))
}

pub async fn get_workspace_users(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(_user): Extension<JwtAuthMiddleware>,
    Extension(workspace): Extension<WorkspaceAuthMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
    let workspace_users = app_state
        .db_client
        .get_workspace_users(workspace.workspace_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = WorkSpaceUserResponseWithRoleDto {
        status: "success",
        data: WorkSpaceUsers {
            users: workspace_users,
        },
    };

    Ok(Json(response))
}

pub async fn update_user_role(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(_user): Extension<JwtAuthMiddleware>,
    Extension(workspace): Extension<WorkspaceAuthMiddleware>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserRoleDto>,
) -> Result<impl IntoResponse, HttpError> {
    payload
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let role_id = app_state
        .db_client
        .get_role_id_by_name(workspace.workspace_id, payload.role_name)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    app_state
        .db_client
        .update_workspace_user_role(workspace.workspace_id, user_id, role_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = Response {
        status: "success",
        message: "User role updated successfully".to_string(),
    };

    Ok(Json(response))
}
