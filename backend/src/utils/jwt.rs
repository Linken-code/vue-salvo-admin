use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

const JWT_SECRET: &[u8] = b"your-secret-key";
const TOKEN_EXPIRE_IN: u64 = 24 * 60 * 60; // 24 hours

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i64, // user_id
    exp: u64, // expiration time
}

pub fn generate_token(user_id: i64) -> String {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + TOKEN_EXPIRE_IN;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
    .unwrap()
}

pub fn verify_token(token: &str) -> Option<i64> {
    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    ) {
        Ok(token_data) => Some(token_data.claims.sub),
        Err(_) => None,
    }
}
