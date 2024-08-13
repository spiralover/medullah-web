use crate::prelude::OnceLockHelper;

pub struct Password;

impl Password {
    pub fn hash(password: String) -> String {
        let salt = crate::MEDULLAH.app().app_key.clone();
        let config = argon2::Config::default();

        argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap()
    }

    pub fn verify(hash: &str, password: &str) -> bool {
        argon2::verify_encoded(hash, password.as_bytes()).unwrap()
    }
}
