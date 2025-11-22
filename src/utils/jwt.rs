use jsonwebtoken::{encode, Header, EncodingKey};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
}

pub fn create_jwt(user_id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
    let claims = Claims { sub: user_id.to_string() };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
}
