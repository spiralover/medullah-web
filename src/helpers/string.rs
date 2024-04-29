use crate::prelude::OnceLockHelper;

#[cfg(feature = "feat-crypto")]
pub fn password_hash(password: String) -> String {
    let salt = crate::MEDULLAH.app().app_key.clone();
    let config = argon2::Config::default();

    argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap()
}

#[cfg(feature = "feat-crypto")]
pub fn password_verify(hash: &str, password: &str) -> bool {
    argon2::verify_encoded(hash, password.as_bytes()).unwrap()
}

#[cfg(feature = "feat-regex")]
pub fn is_username_valid(name: String) -> Box<fancy_regex::Result<bool>> {
    let regex = fancy_regex::Regex::new(r"^[a-z\d](?:[a-z\d]|-(?=[a-z\d])){0,38}$").unwrap();
    Box::new(regex.is_match(name.as_str()))
}
