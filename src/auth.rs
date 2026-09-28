//! Password hashing (argon2), session cookies, and the auth middleware.
//!
//! Single-admin model: the admin account is created from `ADMIN_USER` /
//! `ADMIN_PASS` on first run (see [`crate::db`]). Every request carries a
//! random session token in an HttpOnly cookie; the token is looked up in the
//! `sessions` table on each request.

use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use axum::extract::State;
use axum::{
    extract::Request,
    http::{
        header::{COOKIE, SET_COOKIE},
        HeaderMap, HeaderValue, StatusCode,
    },
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use rand::thread_rng;

use crate::db::Db;
use crate::AppState;

/// Authenticated user, inserted into request extensions by [`auth_middleware`].
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: String,
    pub username: String,
}

/// Hash a password with argon2id and a random salt; returns the PHC string.
pub fn hash_password(password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let salt = SaltString::generate(&mut thread_rng());
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|error| format!("password hashing failed: {error}"))?;
    Ok(hash.to_string())
}

/// Verify a password against a stored PHC hash. Never panics on bad input.
pub fn verify_password(hash: &str, password: &str) -> bool {
    let Ok(parsed) = PasswordHash::new(hash) else {
        return false;
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

/// Pull the `session` cookie value out of request headers, if present.
pub fn session_token_from_headers(headers: &HeaderMap) -> Option<String> {
    let cookie = headers.get(COOKIE)?.to_str().ok()?;
    cookie.split(';').find_map(|part| {
        part.trim()
            .strip_prefix("session=")
            .map(|value| value.to_string())
    })
}

/// Look up the user behind a session token.
pub fn user_for_token(db: &Db, token: &str) -> Option<AuthUser> {
    db.user_for_token(token)
        .ok()?
        .map(|(id, username)| AuthUser { id, username })
}

/// Create a session row and return the new token.
pub fn create_session(db: &Db, user_id: &str) -> Result<String, Box<dyn std::error::Error>> {
    db.create_session(user_id)
}

/// `Set-Cookie` value for a fresh login.
pub fn session_cookie(token: &str) -> String {
    format!("session={token}; Path=/; HttpOnly; SameSite=Lax; Max-Age=2592000")
}

/// `Set-Cookie` value that clears the session cookie (logout).
pub fn clear_session_cookie() -> String {
    "session=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0".to_string()
}

fn set_cookie_header(response: &mut Response, value: &str) {
    if let Ok(header_value) = HeaderValue::from_str(value) {
        response.headers_mut().insert(SET_COOKIE, header_value);
    }
}

/// Attach a session cookie to a redirect response (used by login/logout).
pub fn redirect_with_cookie(target: &str, cookie: &str) -> Response {
    let mut response = Redirect::to(target).into_response();
    set_cookie_header(&mut response, cookie);
    response
}

/// Middleware guarding every route except `/login` and `/static` (those are
/// registered after this layer, so it never sees them).
///
/// API routes (`/api/*`) get a plain 401 so htmx/fetch can detect it;
/// page routes redirect to `/login`.
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Response {
    let user = match session_token_from_headers(req.headers()) {
        Some(token) => user_for_token(&state.db, &token),
        None => None,
    };
    match user {
        Some(user) => {
            req.extensions_mut().insert(user);
            next.run(req).await
        }
        None if req.uri().path().starts_with("/api/") => {
            (StatusCode::UNAUTHORIZED, "unauthorized").into_response()
        }
        None => Redirect::to("/login").into_response(),
    }
}
