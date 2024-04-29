use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::MEDULLAH;
use crate::prelude::OnceLockHelper;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    // The time this claim is generated (timestamp)
    pub iat: usize,
    // Expiry time in timestamp
    pub exp: usize,
}

#[derive(Serialize, Debug)]
pub struct AuthTokenData {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

pub fn generate_token(
    payload: String,
    header: Option<Header>,
    lifetime: Option<i64>,
) -> AuthTokenData {
    let token_lifetime_in_minutes: i64 =
        lifetime.unwrap_or_else(|| MEDULLAH.app().auth_token_lifetime);

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    #[allow(deprecated)]
    let exp = (now + Duration::minutes(token_lifetime_in_minutes)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: payload,
        exp,
        iat,
    };

    let token_header = header.unwrap_or_default();

    let token = encode(
        &token_header,
        &claims,
        &EncodingKey::from_secret(MEDULLAH.app().app_key.clone().as_ref()),
    )
    .unwrap();

    AuthTokenData {
        access_token: token,
        token_type: "bearer".to_string(),
        expires_in: token_lifetime_in_minutes,
    }
}

impl TokenClaims {
    pub fn is_usable(&self) -> bool {
        self.exp > Utc::now().timestamp() as usize
    }
}
