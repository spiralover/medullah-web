/// A utility struct for working with regular expressions for username validation.
pub struct Regex;

/// Enum representing different types of regex patterns for username validation.
pub enum RegexType {
    /// Allows only lowercase letters (1 to 38 characters).
    AlphaNumeric,

    /// Allows lowercase letters, digits, and hyphens (`-`), but no consecutive or trailing hyphens.
    AlphaNumericDash,

    /// Allows lowercase letters, digits, and dots (`.`), but no consecutive or trailing dots.
    AlphaNumericDot,

    /// Allows lowercase letters, digits, hyphens (`-`), and dots (`.`), but no consecutive dots, hyphens, or trailing dots.
    AlphaNumericDashDot,

    /// Allows lowercase letters, digits, and underscores (`_`), but no consecutive or trailing underscores.
    AlphaNumericUnderscore,

    /// Allows lowercase letters, digits, dots (`.`), and underscores (`_`), but no consecutive dots or underscores, and no trailing dots or underscores.
    AlphaNumericDotUnderscore,

    /// Provide your custom regex
    Custom(&'static str),
}

impl Regex {
    /// Validates a string using a specified regex pattern.
    ///
    /// # Parameters
    /// - `val`: A string slice (`&str`) representing the value to validate.
    /// - `rt`: The `RegexType` enum variant that defines which regex pattern to use for validation.
    ///
    /// # Returns
    /// A `Box<Result<bool, fancy_regex::Error>>`, where:
    /// - `Ok(true)` means the string matches the regex pattern (valid username).
    /// - `Ok(false)` means the string does not match the regex pattern (invalid username).
    /// - `Err(fancy_regex::Error)` means there was an error compiling or executing the regex.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use medullah_web::helpers::{Regex, RegexType};
    ///
    /// let valid_username = "user_name123";
    /// let result = Regex::validate(valid_username, RegexType::AlphaNumericUnderscore);
    /// assert_eq!(result.is_ok() && result.unwrap(), true);
    ///
    /// let invalid_username = "user@@";
    /// let result = Regex::validate(invalid_username, RegexType::AlphaNumeric);
    /// assert_eq!(result.is_ok() && result.unwrap(), false);
    /// ```
    pub fn validate(val: &str, rt: RegexType) -> Box<Result<bool, fancy_regex::Error>> {
        let regex = Regex::acquire_regex(rt);
        let regex = match fancy_regex::Regex::new(regex) {
            Ok(regex) => regex,
            Err(err) => return Box::new(Err(err)),
        };

        Box::new(regex.is_match(val))
    }

    /// Validates a username using a specified regex type. This method accepts a `Cow<str>` so it can handle both
    /// `&str` and `String` inputs.
    ///
    /// # Parameters
    /// - `val`: A `Cow<str>` representing the value to validate (can be either a string slice or a `String`).
    /// - `rt`: The `RegexType` enum variant that defines which regex pattern to use for validation.
    ///
    /// # Returns
    /// A `Box<Result<bool, fancy_regex::Error>>` indicating whether the username is valid according to the specified regex.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use medullah_web::helpers::Regex;
    ///
    /// let valid_username = "user.first";
    /// let result = Regex::validate_username(valid_username);
    /// assert_eq!(result.is_ok() && result.unwrap(), true);
    ///
    /// let invalid_username = "user#123";
    /// let result = Regex::validate_username(invalid_username);
    /// assert_eq!(result.is_ok() && result.unwrap(), false);
    /// ```
    pub fn validate_username(val: &str) -> Box<Result<bool, fancy_regex::Error>> {
        Self::validate(val, RegexType::AlphaNumericDot)
    }

    /// Retrieves the regex pattern associated with the given `RegexType` variant.
    ///
    /// # Parameters
    /// - `rt`: The `RegexType` enum variant.
    ///
    /// # Returns
    /// A string slice (`&'static str`) containing the regex pattern associated with the `RegexType` variant.
    fn acquire_regex(rt: RegexType) -> &'static str {
        match rt {
            RegexType::AlphaNumeric => r"^[a-z]{1,38}$", // Only lowercase letters, 1-38 characters.
            RegexType::AlphaNumericDash => r"^[a-z](?!.*\-\-)(?!.*\-$)[a-z\d\-]{0,37}$", // Letters, digits, and dashes, no consecutive or trailing dashes.
            RegexType::AlphaNumericDot => r"^[a-z](?!.*\.\.)(?!.*\.$)[a-z\d\.]{0,37}$", // Letters, digits, and dots, no consecutive or trailing dots.
            RegexType::AlphaNumericUnderscore => r"^[a-z](?!.*\_\_)(?!.*\_$)[a-z\d\_]{0,37}$", // Letters, digits, and underscores, no consecutive or trailing underscores.
            RegexType::AlphaNumericDotUnderscore => r"^[a-z](?!.*\.\.)(?!.*\.$)[a-z\d\._]{0,37}$", // Letters, digits, dots, and underscores.
            RegexType::AlphaNumericDashDot => {
                r"^[a-z](?!.*\-\-)(?!.*\.\.)(?!.*\-$)(?!.*\.$)[a-z\d\-\.\_]{0,37}$"
            } // Letters, digits, dashes, dots, and underscores.
            RegexType::Custom(val) => val,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test for AlphaNumeric regex type
    #[test]
    fn test_alpha_numeric_valid() {
        let result = Regex::validate("username", RegexType::AlphaNumeric);
        assert!(result.is_ok() && result.unwrap());
    }

    #[test]
    fn test_alpha_numeric_invalid() {
        let result = Regex::validate("user123!", RegexType::AlphaNumeric);
        assert!(result.is_ok() && !result.unwrap());

        let result = Regex::validate("user123_", RegexType::AlphaNumeric);
        assert!(result.is_ok() && !result.unwrap());

        let result = Regex::validate("123user", RegexType::AlphaNumeric);
        assert!(result.is_ok() && !result.unwrap());
    }

    // Test for AlphaNumericDash regex type
    #[test]
    fn test_alpha_numeric_dash_valid() {
        let result = Regex::validate("username-123", RegexType::AlphaNumericDash);
        assert!(result.is_ok() && result.unwrap());

        let result = Regex::validate("user-123", RegexType::AlphaNumericDash);
        assert!(result.is_ok() && result.unwrap());
    }

    #[test]
    fn test_alpha_numeric_dash_invalid() {
        let result = Regex::validate("user--123", RegexType::AlphaNumericDash);
        assert!(result.is_ok() && !result.unwrap());

        let result = Regex::validate("user-", RegexType::AlphaNumericDash);
        assert!(result.is_ok() && !result.unwrap());

        let result = Regex::validate("-user123", RegexType::AlphaNumericDash);
        assert!(result.is_ok() && !result.unwrap());
    }

    // Test for AlphaNumericDot regex type
    #[test]
    fn test_alpha_numeric_dot_valid() {
        let result = Regex::validate("user.name", RegexType::AlphaNumericDot);
        assert!(result.is_ok() && result.unwrap());

        let result = Regex::validate("user123.name", RegexType::AlphaNumericDot);
        assert!(result.is_ok() && result.unwrap());
    }

    #[test]
    fn test_alpha_numeric_dot_invalid() {
        let result = Regex::validate("user..name", RegexType::AlphaNumericDot);
        assert!(result.is_ok() && !result.unwrap());

        let result = Regex::validate("user.name.", RegexType::AlphaNumericDot);
        assert!(result.is_ok() && !result.unwrap());

        let result = Regex::validate(".username", RegexType::AlphaNumericDot);
        assert!(result.is_ok() && !result.unwrap());
    }

    // Test for AlphaNumericDashDot regex type
    #[test]
    fn test_alpha_numeric_dash_dot_valid() {
        let result = Regex::validate("user-name.123", RegexType::AlphaNumericDashDot);
        assert!(result.is_ok() && result.unwrap());

        let result = Regex::validate("user-name_123", RegexType::AlphaNumericDashDot);
        assert!(result.is_ok() && result.unwrap());
    }

    #[test]
    fn test_alpha_numeric_dash_dot_invalid() {
        let result = Regex::validate("user..name", RegexType::AlphaNumericDashDot);
        assert!(result.is_ok() && !result.unwrap());

        let result = Regex::validate("user-name.", RegexType::AlphaNumericDashDot);
        assert!(result.is_ok() && !result.unwrap());

        let result = Regex::validate("user-.name", RegexType::AlphaNumericDashDot);
        assert!(result.is_ok() && result.unwrap());
    }

    // Test for AlphaNumericUnderscore regex type
    #[test]
    fn test_alpha_numeric_underscore_valid() {
        let result = Regex::validate("user_name", RegexType::AlphaNumericUnderscore);
        assert!(result.is_ok() && result.unwrap());

        let result = Regex::validate("user123_name", RegexType::AlphaNumericUnderscore);
        assert!(result.is_ok() && result.unwrap());
    }

    #[test]
    fn test_alpha_numeric_underscore_invalid() {
        let result = Regex::validate("user__name", RegexType::AlphaNumericUnderscore);
        assert!(result.is_ok() && !result.unwrap());

        let result = Regex::validate("user_name_", RegexType::AlphaNumericUnderscore);
        assert!(result.is_ok() && !result.unwrap());

        let result = Regex::validate("_username", RegexType::AlphaNumericUnderscore);
        assert!(result.is_ok() && !result.unwrap());
    }

    // Test for AlphaNumericDotUnderscore regex type
    #[test]
    fn test_alpha_numeric_dot_underscore_valid() {
        let result = Regex::validate("user.name_123", RegexType::AlphaNumericDotUnderscore);
        assert!(result.is_ok() && result.unwrap());

        let result = Regex::validate("user_123.name", RegexType::AlphaNumericDotUnderscore);
        assert!(result.is_ok() && result.unwrap());
    }

    #[test]
    fn test_alpha_numeric_dot_underscore_invalid() {
        let result = Regex::validate("user..name_123", RegexType::AlphaNumericDotUnderscore);
        assert!(result.is_ok() && !result.unwrap());

        let result = Regex::validate("user_name_.", RegexType::AlphaNumericDotUnderscore);
        assert!(result.is_ok() && !result.unwrap());

        let result = Regex::validate("_user.name", RegexType::AlphaNumericDotUnderscore);
        assert!(result.is_ok() && !result.unwrap());
    }
}
