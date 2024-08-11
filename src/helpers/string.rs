use uuid::Uuid;

pub struct Str;

impl Str {
    #[cfg(feature = "feat-regex")]
    pub fn is_username_valid(name: String) -> Box<fancy_regex::Result<bool>> {
        let regex = fancy_regex::Regex::new(r"^[a-z][a-z\d\.]{0,37}$").unwrap();
        Box::new(regex.is_match(name.as_str()))
    }

    /// Generate uuid v4 based id with dashes(-) removed
    pub fn uuid() -> String {
        Uuid::new_v4().to_string().replace("-", "")
    }
}