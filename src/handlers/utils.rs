use crate::handlers::utils;
use actix_web::{dev::ServiceRequest, error::InternalError, Error, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use anyhow::anyhow;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const JWT_SECRET_KEY: &[u8] = b"SECRET_KEY";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub async fn create_jwt(username: String) -> Result<String, anyhow::Error> {
    let duration = match Utc::now().checked_add_signed(Duration::minutes(10)) {
        Some(duration) => duration,
        None => {
            return Err(anyhow!("Internal Server Error").into());
        }
    };
    let expiration = duration.timestamp() as usize;
    let claims = Claims {
        sub: username,
        exp: expiration,
    };
    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(utils::JWT_SECRET_KEY),
    ) {
        Ok(token) => Ok(token),
        Err(_) => Err(anyhow!("Internal Server Error").into()),
    }
}

pub async fn jwt_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    match get_token_from_credentials(credentials).await {
        Ok(_) => Ok(req),
        Err(_) => {
            let unauthorized_response = HttpResponse::Unauthorized().finish();
            Err((
                InternalError::from_response("Unauthorized", unauthorized_response).into(),
                req,
            ))
        }
    }
}

pub async fn validate_user_jwt(credentials: BearerAuth, username: String) -> bool {
    let token = get_token_from_credentials(credentials).await;
    token.is_ok() && token.unwrap().claims.sub == username
}

pub async fn get_token_from_credentials(
    credentials: BearerAuth,
) -> std::result::Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        credentials.token(),
        &DecodingKey::from_secret(utils::JWT_SECRET_KEY.as_ref()),
        &Validation::default(),
    )
}

pub fn sha256(plaintext: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(plaintext);
    format!("{:x}", hasher.finalize())
}

pub fn generate_random_string(length: usize) -> String {
    let random_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    random_string
}
