use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use rand::{distributions::Alphanumeric, Rng};
use sha2::{Digest, Sha256};

pub const CSRF_COOKIE_NAME: &str = "csrf_token";

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
    cookie.set_same_site(SameSite::Strict);
    cookie.set_path("/");

    let updated_jar = jar.add(cookie);
    (updated_jar, raw_token)
}

pub fn verify_csrf_token(jar: &CookieJar, provided_token: &str) -> bool {
    if let Some(cookie) = jar.get(CSRF_COOKIE_NAME) {
        let hashed_input = hash_token(provided_token);
        hashed_input == cookie.value()
    } else {
        false
    }
}
