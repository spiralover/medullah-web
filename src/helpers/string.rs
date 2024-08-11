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

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "feat-regex")]
    #[test]
    fn test_is_username_valid_valid_usernames() {
        assert_eq!(Str::is_username_valid("a".to_string()).unwrap(), true);
        assert_eq!(Str::is_username_valid("abc1234".to_string()).unwrap(), true);
        assert_eq!(Str::is_username_valid("a.b.c".to_string()).unwrap(), true);
        assert_eq!(Str::is_username_valid("username1".to_string()).unwrap(), true);
        assert_eq!(Str::is_username_valid("a123456789012345678901234567890123".to_string()).unwrap(), true); // 37 chars
    }

    #[cfg(feature = "feat-regex")]
    #[test]
    fn test_is_username_valid_invalid_usernames() {
        assert_eq!(Str::is_username_valid("1username".to_string()).unwrap(), false); // Starts with a digit
        assert_eq!(Str::is_username_valid("username!".to_string()).unwrap(), false); // Invalid character
        assert_eq!(Str::is_username_valid("".to_string()).unwrap(), false); // Empty username
        assert_eq!(Str::is_username_valid("a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.q.r.s.t.u.v.w.x.y.z".to_string()).unwrap(), false); // More than 37 chars
    }

    #[test]
    fn test_uuid() {
        let uuid = Str::uuid();
        // Check if the length is 32 (UUID v4 without dashes)
        assert_eq!(uuid.len(), 32);
        // Check if it contains only hexadecimal characters
        assert!(uuid.chars().all(|c| c.is_digit(16)));

        // Generate a few UUIDs and check that they are unique
        let uuid_set: std::collections::HashSet<_> = (0..1000).map(|_| Str::uuid()).collect();
        assert_eq!(uuid_set.len(), 1000); // Check for uniqueness
    }
}