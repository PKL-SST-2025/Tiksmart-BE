use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use rand::{distributions::Alphanumeric, Rng};
use sha2::{Digest, Sha256};

pub const CSRF_COOKIE_NAME: &str = "csrf-token";

pub fn generate_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

pub fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn create_csrf_cookie(jar: CookieJar) -> (CookieJar, String) {
    let raw_token = generate_token();
    let hashed_token = hash_token(&raw_token);
        
    let mut cookie = Cookie::new(CSRF_COOKIE_NAME, hashed_token);
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_path("/");

    let updated_jar = jar.add(cookie);
    (updated_jar, raw_token)
}


pub fn verify_csrf_token(jar: &CookieJar, raw_token: &str) -> bool {
    let Some(cookie) = jar.get("csrf-token") else { // Check cookie name
        tracing::debug!("CSRF cookie not found.");
        return false;
    };
    let stored_hashed_token = cookie.value();

    // Re-hash the raw_token received from the client
    let hashed_received_token = hash_token(raw_token); // Ensure this hash_token matches the one in create_csrf_cookie

    // Compare the newly hashed client token with the stored hashed token
    hashed_received_token == stored_hashed_token
}