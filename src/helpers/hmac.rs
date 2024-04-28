use chrono::Utc;
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub fn hmac_hash(value: String, secret: String) -> String {
    type HmacSha256 = Hmac<Sha256>;

    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");

    mac.update(value.as_bytes());

    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    hex::encode(code_bytes.as_slice())
}

pub fn hmac_generate_random() -> String {
    type HmacSha256 = Hmac<Sha256>;

    let timestamp = Utc::now().timestamp().to_string();

    let mut mac =
        HmacSha256::new_from_slice(timestamp.as_bytes()).expect("HMAC can take key of any size");

    mac.update(timestamp.as_bytes());

    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    hex::encode(code_bytes.as_slice())
}
