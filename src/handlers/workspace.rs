use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::Path,
    http::{HeaderMap, header},
    response::IntoResponse,
};
use axum_extra::extract::cookie::Cookie;
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState,
    constants::permissions,
    database::workspace::WorkspaceExt,
    dtos::{
        Response,
        role::UpdateRoleDto,
        workspace::{
            WorkspaceCreateDto, WorkspaceCreateResponseDto, WorkspaceList, WorkspaceListResponse,
            WorkspaceWithRoleAndPermissions,
        },
    },
    error::HttpError,
    middleware::{
        jwt_auth_middleware::JwtAuthMiddleware, workspace_middleware::WorkspaceAuthMiddleware,
    },
    workspace_auth,
};

pub fn workspace_handler() -> axum::Router {
    axum::Router::new()
        .route("/create", axum::routing::post(create_workspace))
        .route(
            "/update",
            axum::routing::put(update_workspace)
                .layer(workspace_auth!(permissions::UPDATE_WORKSPACE)),
        )
        .route(
            "/delete",
            axum::routing::delete(delete_workspace)
                .layer(workspace_auth!(permissions::DELETE_WORKSPACE)),
        )
        .route("/", axum::routing::get(get_all_workspace))
        .route("/{workspace_id}", axum::routing::get(get_workspace_by_id))
}

pub fn create_workspace_response(
    workspace_data: WorkspaceWithRoleAndPermissions,
    app_state: Arc<AppState>,
) -> Result<impl IntoResponse, HttpError> {
    let mut headers = HeaderMap::new();

    let cookie_duration = time::Duration::minutes(app_state.env.jwt_maxage * 60);

    let workspace_cookie = Cookie::build(("workspace", workspace_data.workspace.id.to_string()))
        .path("/")
        .http_only(true)
        .max_age(cookie_duration)
        .build();

    headers.append(
        header::SET_COOKIE,
        workspace_cookie.to_string().parse().unwrap(),
    );

    let response = Json(WorkspaceCreateResponseDto {
        status: "success",
        data: workspace_data,
    });

    let mut response = response.into_response();

    response.headers_mut().extend(headers);

    Ok(response)
}

pub async fn create_workspace(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JwtAuthMiddleware>,
    Json(payload): Json<WorkspaceCreateDto>,
) -> Result<impl IntoResponse, HttpError> {
    payload
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let workspace = app_state
        .db_client
        .create_workspace(&payload.name, user.user.id)
        .await;

    let workspace = match workspace {
        Ok(workspace) => workspace,
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                return Err(HttpError::unique_constraint_violation(
                    "Workspace with the same name already exists".to_string(),
                ));
            } else {
                return Err(HttpError::server_error("Database error".to_string()));
            }
        }
        Err(e) => return Err(HttpError::server_error(e.to_string())),
    };

    let workspace_with_role_permissions = app_state
        .db_client
        .get_workspace_details(Some(user.user.id), Some(workspace.id))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    create_workspace_response(workspace_with_role_permissions, app_state)
}

pub async fn update_workspace(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(_user): Extension<JwtAuthMiddleware>,
    Extension(workspace): Extension<WorkspaceAuthMiddleware>,
    Json(payload): Json<UpdateRoleDto>,
) -> Result<impl IntoResponse, HttpError> {
    payload
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let workspace = app_state
        .db_client
        .update_workspace(workspace.workspace_id, &payload.name)
        .await;

    if let Err(sqlx::Error::Database(db_err)) = workspace {
        if db_err.is_unique_violation() {
            return Err(HttpError::unique_constraint_violation(
                "Workspace with the same name already exists".to_string(),
            ));
        } else {
            return Err(HttpError::server_error("Database error".to_string()));
        }
    }

    if let Err(e) = workspace {
        return Err(HttpError::server_error(e.to_string()));
    }

    let response = Response {
        status: "success",
        message: "Workspace updated successfully".to_string(),
    };

    Ok(Json(response))
}

pub async fn delete_workspace(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(_user): Extension<JwtAuthMiddleware>,
    Extension(workspace): Extension<WorkspaceAuthMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
    app_state
        .db_client
        .delete_workspace(workspace.workspace_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = Response {
        status: "success",
        message: "Workspace deleted successfully".to_string(),
    };

    Ok(Json(response))
}

pub async fn get_all_workspace(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JwtAuthMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
    let workspaces = app_state
        .db_client
        .get_all_user_workspace(user.user.id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = WorkspaceListResponse {
        status: "success",
        data: WorkspaceList { workspaces },
    };

    Ok(Json(response))
}

pub async fn get_workspace_by_id(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JwtAuthMiddleware>,
    Path(workspace_id): Path<Uuid>,
) -> Result<impl IntoResponse, HttpError> {
    let workspace = app_state
        .db_client
        .get_workspace_details(Some(user.user.id), Some(workspace_id))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    create_workspace_response(workspace, app_state)
}
