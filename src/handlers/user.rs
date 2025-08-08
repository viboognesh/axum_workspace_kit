use std::sync::Arc;

use axum::{Extension, Json, extract::Query, response::IntoResponse};
use chrono::{Duration, Utc};
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState,
    database::{auth::AuthExt, user::UserExt},
    dtos::{
        Response,
        user::{
            FilterUserDto, UserData, UserEmailChangeRequest, UserEmailChangeVerificationDto,
            UserPasswordUpdate, UserResponse,
        },
    },
    error::HttpError,
    mail::mail::send_email_change_notification,
    middleware::jwt_auth_middleware::JwtAuthMiddleware,
    utils::password,
};

pub fn user_handler() -> axum::Router {
    axum::Router::new()
        .route("/me", axum::routing::get(get_me))
        .route("/update-password", axum::routing::put(update_user_password))
        .route("/change-email", axum::routing::put(change_email_request))
        .route("/verify-email", axum::routing::get(verify_email_change))
}

pub async fn get_me(
    Extension(_app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JwtAuthMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
    let filtered_user = FilterUserDto::filter_user(&user.user);
    let response = UserResponse {
        status: "success",
        data: UserData {
            user: filtered_user,
        },
    };

    Ok(Json(response))
}

pub async fn update_user_password(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JwtAuthMiddleware>,
    Json(payload): Json<UserPasswordUpdate>,
) -> Result<impl IntoResponse, HttpError> {
    payload
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let user = user.user;

    let password_match = password::compare(&payload.current_password, &user.password)
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    if !password_match {
        return Err(HttpError::unauthorized(
            "Current password is incorrect".to_string(),
        ));
    }

    let hash_password = password::hash_password(&payload.new_password)
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    app_state
        .db_client
        .update_user_password(user.id, &hash_password)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = Response {
        status: "success",
        message: "Password updated successfully".to_string(),
    };

    Ok(Json(response))
}

pub async fn change_email_request(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JwtAuthMiddleware>,
    Json(payload): Json<UserEmailChangeRequest>,
) -> Result<impl IntoResponse, HttpError> {
    payload
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let user = user.user;
    let token = Uuid::new_v4();
    let token_expires_at = Utc::now() + Duration::days(1);

    let user = app_state
        .db_client
        .update_user_email_request(user.id, payload.email.clone(), token, token_expires_at)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let send_email_result = send_email_change_notification(
        &app_state.mail_config,
        &app_state.env.frontend_base_url,
        &user.email,
        &payload.email,
        &user.name,
        &token.to_string(),
    )
    .await;

    match send_email_result {
        Ok(_) => Ok(Json(Response {
            status: "success",
            message: "Verification link has been sent to your new email address. Please check your inbox to confirm your email change".to_string(),
        })),
        Err(_e) => Err(HttpError::server_error("We were unable to send your email change request. Please try again later".to_string())),
    }
}

pub async fn verify_email_change(
    Query(query_params): Query<UserEmailChangeVerificationDto>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let token = Uuid::parse_str(&query_params.token).unwrap();

    app_state
        .db_client
        .verify_email_change(token)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = Response {
        status: "success",
        message: "Email changed successfully".to_string(),
    };

    Ok(Json(response))
}
