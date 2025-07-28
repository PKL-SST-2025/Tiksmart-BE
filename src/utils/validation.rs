// File: src/utils/validation.rs

use once_cell::sync::Lazy;
use regex::Regex;

// We use `Lazy` from `once_cell` to ensure the regex is compiled only once.
// Compiling a regex is an expensive operation, so we want to do it at the
// start of the program and reuse the compiled object.
static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
});

/// Validates if the provided string is a well-formed email address.
///
/// This function uses a pre-compiled regular expression for performance.
/// It checks for a basic email structure like `user@domain.com`.
///
/// # Arguments
///
/// * `email` - A string slice that holds the email to be validated.
///
/// # Returns
///
/// * `true` if the email format is valid, `false` otherwise.
///
/// # Example
///
/// ```
/// assert_eq!(validate_email("test@example.com"), true);
/// assert_eq!(validate_email("invalid-email"), false);
/// ```
pub fn validate_email(email: &str) -> bool {
    EMAIL_REGEX.is_match(email)
}