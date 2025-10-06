use crate::config::Config;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub email: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(user_id: Uuid, email: String, config: &Config) -> Self {
        let now = Utc::now();
        let expiration = now + Duration::seconds(config.jwt_expiration);

        Self {
            sub: user_id.to_string(),
            email,
            exp: expiration.timestamp(),
            iat: now.timestamp(),
        }
    }
}

pub fn create_token(user_id: Uuid, email: String, config: &Config) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(user_id, email, config);
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
}

pub fn verify_token(token: &str, config: &Config) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(password, hash)
}
