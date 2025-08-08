use std::sync::LazyLock;

use argon2::{
    self, Argon2, Params, PasswordHash, PasswordVerifier,
    password_hash::{Error as PasswordHashError, PasswordHasher, SaltString, rand_core::OsRng},
};

use crate::error::ErrorMessage;

const MAX_PASSWORD_LENGTH: usize = 64;
const MIN_PASSWORD_LENGTH: usize = 8;

static ARGON2: LazyLock<Argon2<'static>> = LazyLock::new(|| {
    let params = Params::new(4096, 3, 1, Some(32))
        .expect("There is some error in setting up params for argon2");

    Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params)
});

pub fn hash_password(password: impl AsRef<[u8]>) -> Result<String, ErrorMessage> {
    let password = password.as_ref();
    if password.is_empty() {
        return Err(ErrorMessage::EmptyPassword);
    }

    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(ErrorMessage::ExceededMaxPasswordLength(password.len()));
    }

    if password.len() < MIN_PASSWORD_LENGTH {
        return Err(ErrorMessage::PasswordTooShort(password.len()));
    }
    let salt = SaltString::generate(&mut OsRng);
    let hash = ARGON2
        .hash_password(password, &salt)
        .map_err(|_| ErrorMessage::HashingError)
        .map(|h| h.to_string())?;
    Ok(hash)
}

pub fn compare(password: impl AsRef<[u8]>, hashed_password: &str) -> Result<bool, ErrorMessage> {
    let password = password.as_ref();
    if password.is_empty() {
        return Err(ErrorMessage::EmptyPassword);
    }

    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(ErrorMessage::ExceededMaxPasswordLength(password.len()));
    }

    if password.len() < MIN_PASSWORD_LENGTH {
        return Err(ErrorMessage::PasswordTooShort(password.len()));
    }
    let parsed_hash =
        PasswordHash::new(hashed_password).map_err(|_| ErrorMessage::InvalidHashFormat)?;

    match ARGON2.verify_password(password, &parsed_hash) {
        Ok(()) => Ok(true),
        Err(PasswordHashError::Password) => Ok(false),
        Err(_) => Err(ErrorMessage::HashingError),
    }
}
