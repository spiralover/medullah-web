use base64::{engine, Engine};

use crate::prelude::AppResult;

pub fn base64_encode(str: &str) -> AppResult<String> {
    Ok(engine::general_purpose::STANDARD.encode(str))
}

pub fn base64_decode(str: &str) -> AppResult<String> {
    Ok(String::from_utf8(
        engine::general_purpose::STANDARD.decode(str)?,
    )?)
}
