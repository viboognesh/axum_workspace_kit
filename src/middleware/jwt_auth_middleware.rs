use std::sync::Arc;

use axum::{Extension, extract::Request, http::header, middleware::Next, response::IntoResponse};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    AppState,
    database::auth::AuthExt,
    error::{ErrorMessage, HttpError},
    models::User,
    utils::token,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtAuthMiddleware {
    pub user: User,
}

pub async fn auth_middleware(
    cookie_jar: CookieJar,
    Extension(app_state): Extension<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, HttpError> {
    let cookie = cookie_jar
        .get("token")
        .map(|c| c.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|h| h.to_str().ok())
                .and_then(|h| {
                    if h.starts_with("Bearer ") {
                        Some(h.trim_start_matches("Bearer ").to_string())
                    } else {
                        None
                    }
                })
        });

    let token = cookie.ok_or(HttpError::unauthorized(
        ErrorMessage::TokenNotProvided.to_string(),
    ))?;

    let token_details = match token::decode_token(&token, app_state.env.jwt_secret.as_bytes()) {
        Ok(details) => details,
        Err(_) => {
            return Err(HttpError::unauthorized(
                ErrorMessage::InvalidToken.to_string(),
            ));
        }
    };

    let user_id = Uuid::parse_str(&token_details)
        .map_err(|_| HttpError::unauthorized(ErrorMessage::InvalidToken.to_string()))?;

    let user = app_state
        .db_client
        .get_user(Some(user_id), None, None)
        .await
        .map_err(|_| HttpError::unauthorized(ErrorMessage::InvalidToken.to_string()))?;

    let user = user.ok_or(HttpError::unauthorized(
        ErrorMessage::UserNoLongerExists.to_string(),
    ))?;

    req.extensions_mut().insert(JwtAuthMiddleware { user });
    Ok(next.run(req).await)
}
