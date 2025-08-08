use std::sync::LazyLock;

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::error::{ErrorMessage, HttpError};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

const DEFAULT_ALGORITHM: Algorithm = Algorithm::HS256;
static DEFAULT_TOKEN_VALIDATION: LazyLock<Validation> =
    LazyLock::new(|| Validation::new(DEFAULT_ALGORITHM));
static HEADER: LazyLock<Header> = LazyLock::new(|| Header::new(DEFAULT_ALGORITHM));

pub fn create_token(
    user_id: &str,
    secret: &[u8],
    expires_in_seconds: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    if user_id.is_empty() {
        return Err(jsonwebtoken::errors::ErrorKind::InvalidSubject.into());
    }

    let now = chrono::Utc::now();

    let claims = TokenClaims {
        sub: user_id.to_string(),
        iat: now.timestamp() as usize,
        exp: (now + chrono::Duration::seconds(expires_in_seconds)).timestamp() as usize,
    };

    jsonwebtoken::encode(&HEADER, &claims, &EncodingKey::from_secret(secret)).map_err(|e| e.into())
}

pub fn decode_token(token: &str, secret: &[u8]) -> Result<String, HttpError> {
    let decode = jsonwebtoken::decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(secret),
        &DEFAULT_TOKEN_VALIDATION,
    );

    match decode {
        Ok(token) => Ok(token.claims.sub),
        Err(_) => Err(HttpError::unauthorized(
            ErrorMessage::InvalidToken.to_string(),
        )),
    }
}
