use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, decode, DecodingKey, encode, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

use crate::MEDULLAH;
use crate::prelude::{AppResult, OnceLockHelper};

pub struct Jwt;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    // The time this claim is generated (timestamp)
    pub iat: usize,
    // Expiry time in timestamp
    pub exp: usize,
    // Issuer
    pub iss: String,
}

#[derive(Serialize, Debug)]
pub struct AuthTokenData {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

impl Jwt {
    pub fn generate(payload: String) -> AppResult<AuthTokenData> {
        let token_lifetime_in_minutes = MEDULLAH.app().auth_token_lifetime;

        let now = Utc::now();
        let iat = now.timestamp() as usize;
        #[allow(deprecated)]
            let exp = (now + Duration::minutes(token_lifetime_in_minutes)).timestamp() as usize;
        let claims: TokenClaims = TokenClaims {
            sub: payload,
            exp,
            iat,
            iss: MEDULLAH.app().app_frontend_url.clone(),
        };

        let token_header = Header::new(Algorithm::RS256);
        let encoding_key = EncodingKey::from_rsa_pem(MEDULLAH.app().app_private_key.as_bytes())?;

        let token = encode(&token_header, &claims, &encoding_key)?;

        Ok(AuthTokenData {
            access_token: token,
            token_type: "bearer".to_string(),
            expires_in: token_lifetime_in_minutes,
        })
    }

    pub fn decode(token: &String) -> AppResult<TokenData<TokenClaims>> {
        Ok(decode::<TokenClaims>(
            token,
            &DecodingKey::from_rsa_pem(MEDULLAH.app().auth_iss_public_key.as_ref())?,
            &Validation::new(Algorithm::RS256),
        )?)
    }
}

impl TokenClaims {
    pub fn is_usable(&self) -> bool {
        self.exp > Utc::now().timestamp() as usize
    }
}
