// File: src/utils/random.rs

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

/// Generates a cryptographically secure, random alphanumeric string of a given length.
///
/// This is useful for creating things like API keys, session IDs, or password reset tokens.
/// It uses `rand::thread_rng`, which is seeded by the operating system.
///
/// # Arguments
///
/// * `len` - The desired length of the random string.
///
/// # Returns
///
/// * A `String` containing the generated random characters.
///
/// # Example
///
/// ```
/// let token = generate_random_token(32);
/// assert_eq!(token.len(), 32);
/// ```
pub fn generate_random_token(len: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}