use axum_extra::extract::cookie::Cookie;
use time::OffsetDateTime;
use tracing::error;
use crate::common::security::jwt::Claims;
use crate::config::app_env::RUN_MODE;

pub fn build_cookie(claims: &Claims, cookie_name: String, value: String, domain: String) -> Cookie<'static> {
    let is_secure = *RUN_MODE != "local";

    let expiration_time = match OffsetDateTime::from_unix_timestamp(claims.exp as i64) {
        Ok(dt) => Some(dt),
        Err(e) => {
            error!("Issue when converting to OffsetDateTime with claims exp: {:?}", e);
            None
        }
    };

    Cookie::build((cookie_name, value))
        .domain(domain)
        .path("/")
        .secure(is_secure)
        .http_only(true)
        .expires(expiration_time)
        .build()
}