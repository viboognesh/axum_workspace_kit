use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::Query,
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::cookie::Cookie;
use chrono::{Duration, Utc};
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState,
    database::{auth::AuthExt, workspace::WorkspaceExt},
    dtos::{
        Response,
        auth::{
            ForgotPasswordDto, LoginResponse, LoginUserDto, RegisterUserDto, ResetPasswordDto,
            UserDataResponse, VerifyEmailQueryDto,
        },
        user::FilterUserDto,
    },
    error::{ErrorMessage, HttpError},
    mail::mail::{send_password_reset_email, send_verification_email, send_welcome_email},
    utils::{password, token},
};

pub fn auth_handler() -> axum::Router {
    axum::Router::new()
        .route("/register", axum::routing::post(register))
        .route("/login", axum::routing::post(login))
        .route("/verify", axum::routing::get(verify_email))
        .route("/forgot-password", axum::routing::post(forgot_password))
        .route("/reset-password", axum::routing::post(reset_password))
}

pub async fn register(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(payload): Json<RegisterUserDto>,
) -> Result<impl IntoResponse, HttpError> {
    payload
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let verification_token = Uuid::new_v4();
    let expires_at = Utc::now() + Duration::hours(24);

    let hash_password = password::hash_password(&payload.password)
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let result = app_state
        .db_client
        .save_user(
            &payload.name,
            &payload.email,
            &hash_password,
            verification_token,
            expires_at,
        )
        .await;

    match result {
        Ok(user) => {
            let send_email_result = send_verification_email(
                &app_state.mail_config,
                &app_state.env.backend_base_url,
                &app_state.mail_config.mail_template_path,
                &user.email,
                &user.name,
                &verification_token.to_string(),
            )
            .await;

            if let Err(_) = send_email_result {
                return Err(HttpError::server_error("We were unable to send your verification email. However, you can login and manually request email verification again from account settings.".to_string()));
            } else {
                Ok((StatusCode::CREATED, Json(Response{status:"success", message:"Registration successful! Please check your email to verify your account".to_string()})))
            }
        }

        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                Err(HttpError::unique_constraint_violation(
                    ErrorMessage::EmailExit.to_string(),
                ))
            } else {
                Err(HttpError::server_error("Database error".to_string()))
            }
        }
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

pub async fn login(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(payload): Json<LoginUserDto>,
) -> Result<impl IntoResponse, HttpError> {
    payload
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let result = app_state
        .db_client
        .get_user(None, None, Some(&payload.email))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let user = result.ok_or(HttpError::bad_request(
        ErrorMessage::WrongeCredentials.to_string(),
    ))?;

    let password_matched = password::compare(&payload.password, &user.password)
        .map_err(|_e| HttpError::bad_request(ErrorMessage::WrongeCredentials.to_string()))?;

    if !password_matched {
        return Err(HttpError::bad_request(
            ErrorMessage::WrongeCredentials.to_string(),
        ));
    }

    let token = token::create_token(
        &user.id.to_string(),
        &app_state.env.jwt_secret.as_bytes(),
        app_state.env.jwt_maxage,
    )
    .map_err(|e| HttpError::server_error(e.to_string()))?;

    let mut headers = HeaderMap::new();

    let cookie_duration = time::Duration::minutes(app_state.env.jwt_maxage * 60);

    let cookie = Cookie::build(("token", token.clone()))
        .path("/")
        .max_age(cookie_duration)
        .http_only(true)
        .build();

    headers.append(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    let workspace = app_state
        .db_client
        .get_workspace_details(Some(user.id), None)
        .await
        .ok();

    if let Some(ref workspace) = workspace {
        let workspace_cookie = Cookie::build(("workpace", workspace.workspace.id.to_string()))
            .path("/")
            .max_age(cookie_duration)
            .http_only(true)
            .build();
        headers.append(
            header::SET_COOKIE,
            workspace_cookie.to_string().parse().unwrap(),
        );
    }

    let filter_user = FilterUserDto::filter_user(&user);

    let response = axum::response::Json(LoginResponse {
        status: "success",
        token,
        data: UserDataResponse {
            user: filter_user,
            workspace,
        },
    });

    let mut response = response.into_response();
    response.headers_mut().extend(headers);
    Ok(response)
}

pub async fn verify_email(
    Query(query_params): Query<VerifyEmailQueryDto>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let token = Uuid::parse_str(&query_params.token).unwrap();

    let result = app_state
        .db_client
        .get_user_id_by_token(token)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let email_verification = result.ok_or(HttpError::bad_request(
        ErrorMessage::InvalidToken.to_string(),
    ))?;

    if Utc::now() > email_verification.expires_at {
        return Err(HttpError::bad_request(
            "Verification token has expired".to_string(),
        ));
    }

    app_state
        .db_client
        .verify_user(email_verification.user_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let db_result = app_state
        .db_client
        .get_user(Some(email_verification.user_id), None, None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let user = db_result.ok_or(HttpError::bad_request(
        ErrorMessage::WrongeCredentials.to_string(),
    ))?;

    let send_welcome_email_result = send_welcome_email(
        &app_state.mail_config,
        &user.email,
        &app_state.env.frontend_base_url,
        &user.name,
    )
    .await;

    if let Err(e) = send_welcome_email_result {
        eprintln!("Failed to send welcome email: {}", e);
        return Err(HttpError::server_error(e.to_string()));
    }

    let token = token::create_token(
        &user.id.to_string(),
        &app_state.env.jwt_secret.as_bytes(),
        app_state.env.jwt_maxage,
    )
    .map_err(|e| HttpError::server_error(e.to_string()))?;

    let mut headers = HeaderMap::new();

    let cookie_duration = time::Duration::minutes(app_state.env.jwt_maxage * 60);

    let cookie = Cookie::build(("token", token.clone()))
        .path("/")
        .max_age(cookie_duration)
        .http_only(true)
        .build();

    headers.append(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    let redirect = Redirect::to(&app_state.env.frontend_base_url);

    let mut response = redirect.into_response();
    response.headers_mut().extend(headers);
    Ok(response)
}

pub async fn forgot_password(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(payload): Json<ForgotPasswordDto>,
) -> Result<impl IntoResponse, HttpError> {
    payload
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let user = app_state
        .db_client
        .get_user(None, None, Some(&payload.email))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?
        .ok_or(HttpError::bad_request(
            ErrorMessage::WrongeCredentials.to_string(),
        ))?;

    let reset_token = Uuid::new_v4();
    let expires_at = Utc::now() + Duration::hours(24);

    app_state
        .db_client
        .save_password_reset_token(user.id, reset_token, expires_at)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let send_password_reset_email_result = send_password_reset_email(
        &app_state.mail_config,
        &user.email,
        &app_state.env.frontend_base_url,
        &user.name,
        &reset_token.to_string(),
    )
    .await;

    if let Err(e) = send_password_reset_email_result {
        eprintln!("Failed to send password reset email: {}", e);
        return Err(HttpError::server_error(e.to_string()));
    }

    Ok((
        StatusCode::OK,
        Json(Response {
            status: "success",
            message: "Password reset email sent".to_string(),
        }),
    ))
}

pub async fn reset_password(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(payload): Json<ResetPasswordDto>,
) -> Result<impl IntoResponse, HttpError> {
    payload
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let reset_token = Uuid::parse_str(&payload.token)
        .map_err(|_e| HttpError::bad_request(ErrorMessage::InvalidToken.to_string()))?;

    let password_reset = app_state
        .db_client
        .get_password_reset_token(reset_token)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?
        .ok_or(HttpError::bad_request(
            ErrorMessage::WrongeCredentials.to_string(),
        ))?;

    if Utc::now() > password_reset.expires_at {
        return Err(HttpError::bad_request(
            "Password reset token has expired".to_string(),
        ));
    }

    let hash_password = password::hash_password(&payload.password)
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    app_state
        .db_client
        .reset_password(password_reset.user_id, &hash_password)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = Json(Response {
        status: "success",
        message: "Password reset successful".to_string(),
    });

    Ok(response)
}
