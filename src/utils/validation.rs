// File: src/utils/validation.rs

use std::borrow::Cow;

use lazy_static::lazy_static;
use regex::Regex;
use rust_decimal::Decimal;
use validator::{Validate, ValidationError};

// --- Pre-compiled Regular Expressions for Performance ---

lazy_static! {
    /// Regex for a valid email format.
    pub static ref EMAIL_REGEX: Regex = Regex::new(
        r"(?i)^[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?$"
    ).unwrap();

    /// Regex for a strong password.
    /// - At least 8 characters
    /// - At least one uppercase letter
    /// - At least one lowercase letter
    /// - At least one number
    /// - At least one special character
    pub static ref PASSWORD_REGEX: Regex = Regex::new(
        r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$"
    ).unwrap();

    /// Regex for a "slug-friendly" or username-like string.
    /// - Starts with a letter
    /// - Contains only alphanumeric characters, hyphens, and underscores
    /// - 3 to 30 characters long
    pub static ref SLUG_REGEX: Regex = Regex::new(
        r"^[a-zA-Z][a-zA-Z0-9_-]{2,29}$"
    ).unwrap();

    /// Regex to check for potentially harmful characters or basic script injection.
    /// This is a basic safeguard and should not be the only line of defense (use output encoding!).
    /// It blocks characters like <, >, (, ), {, }, &, and ".
    pub static ref DANGEROUS_CHARS_REGEX: Regex = Regex::new(
        r#"[<>()"{}&]"#
    ).unwrap();
}


// --- Generic Validation Functions ---

/// Validates an email address against the standard email format.
pub fn is_valid_email(email: &str) -> Result<(), ValidationError> {
    if EMAIL_REGEX.is_match(email) {
        Ok(())
    } else {
        // Return a proper ValidationError on failure
        Err(ValidationError::new("invalid_email"))
    }
}

/// Validates password strength based on the predefined `PASSWORD_REGEX`.
///
/// It now returns `Result<(), ValidationError>`, which allows us to return
/// detailed error messages directly from the function.
pub fn is_strong_password(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 {
        let mut err = ValidationError::new("password_too_short");
        err.message = Some(Cow::from("Password must be at least 8 characters long."));
        return Err(err);
    }
    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        let mut err = ValidationError::new("password_no_lowercase");
        err.message = Some(Cow::from("Password must contain at least one lowercase letter."));
        return Err(err);
    }
    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        let mut err = ValidationError::new("password_no_uppercase");
        err.message = Some(Cow::from("Password must contain at least one uppercase letter."));
        return Err(err);
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        let mut err = ValidationError::new("password_no_digit");
        err.message = Some(Cow::from("Password must contain at least one number."));
        return Err(err);
    }
    if password.chars().all(|c| c.is_alphanumeric()) {
        let mut err = ValidationError::new("password_no_special_char");
        err.message = Some(Cow::from("Password must contain at least one special character."));
        return Err(err);
    }
    Ok(())
}

/// Checks if a string is within a specified min/max length and free of dangerous characters.
/// This is a general-purpose validator for user-provided text like names, descriptions, etc.
///
/// # Arguments
/// * `field_name` - The name of the field being validated (for clear error messages).
/// * `text` - The string slice to validate.
/// * `min_len` - The minimum allowed length.
/// * `max_len` - The maximum allowed length.
pub fn is_valid_text(
    field_name: &str,
    text: &str,
    min_len: usize,
    max_len: usize,
) -> Result<(), String> {
    if text.len() < min_len {
        return Err(format!(
            "{} must be at least {} characters long.",
            field_name, min_len
        ));
    }
    if text.len() > max_len {
        return Err(format!(
            "{} must not exceed {} characters.",
            field_name, max_len
        ));
    }
    if DANGEROUS_CHARS_REGEX.is_match(text) {
        return Err(format!(
            "{} contains invalid characters (e.g., <, >, &).",
            field_name
        ));
    }
    Ok(())
}

/// Checks if a string contains any of the characters blacklisted in `DANGEROUS_CHARS_REGEX`.
/// It returns an `Err` if a dangerous character is found, and `Ok` otherwise.
/// This is the correct pattern for a "must not match" style validation.
pub fn is_safe_text(text: &str) -> Result<(), ValidationError> {
    if DANGEROUS_CHARS_REGEX.is_match(text) {
        // A match was found, which is an error in this case.
        let mut err = ValidationError::new("invalid_characters");
        err.message = Some(Cow::from("Input contains invalid characters (e.g., <, >, &, etc.)."));
        Err(err)
    } else {
        // No match found, so the text is safe.
        Ok(())
    }
}

/// Validates a string to ensure it's suitable for use as a username, slug, or unique key.
pub fn is_valid_slug(slug: &str) -> bool {
    SLUG_REGEX.is_match(slug)
}


// --- Validation using the `validator` crate for structs ---

/// A generic function to run validation on any struct that derives `validator::Validate`.
/// This is extremely useful for validating entire request payloads (DTOs) at once.
///
/// # Returns
/// `Ok(())` if validation succeeds, or an `AppError::BadRequest` with a detailed
/// message if it fails.
pub fn validate_payload<T: Validate>(payload: &T) -> Result<(), crate::errors::AppError> {
    payload
        .validate()
        .map_err(|e| crate::errors::AppError::BadRequest(e.to_string()))
}


/// Custom validation function to check if a Decimal is non-negative.
pub fn is_non_negative_decimal(price: &Decimal) -> Result<(), ValidationError> {
    if price.is_sign_negative() {
        let mut err = ValidationError::new("negative_price");
        err.message = Some(Cow::from("Price cannot be negative."));
        Err(err)
    } else {
        Ok(())
    }
}