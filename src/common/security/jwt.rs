use std::collections::HashSet;
use std::str::FromStr;
use axum::body::Body;
use axum::extract::{State};
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use chrono::{DateTime, Utc};
use http::{Request, header};
use jsonwebtoken::{Algorithm, decode, encode, Header, TokenData, Validation};
use jsonwebtoken::errors::ErrorKind;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, trace};
use uuid::Uuid;
use crate::common::errors::authentication_error::AuthenticationError;
use crate::common::models::models::Identity;
use crate::config::app_env::{JWT_AUD, JWT_EXP, JWT_ISS};
use crate::config::AppState;
use crate::config::jwt_config::{KEYS};
use crate::domains::users::db_models::SwiftUser;


lazy_static! {
    static ref VALIDATION: Validation = {
        let mut validation = Validation::default();
        validation.leeway = 10;
        validation.validate_exp = true;
        validation.validate_nbf = true;
        validation.algorithms = vec![Algorithm::HS256];

        let mut auds = HashSet::with_capacity(1);
        auds.insert(JWT_AUD.clone());
        validation.aud = Some(auds);
        validation
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    jti: String,
    sub: String,
    iss: String,
    aud: Vec<String>,
    iat: usize,
    pub exp: usize,
    org_access: Vec<Uuid>,
}

pub async fn authenticate(
    cookie_jar: CookieJar,
    State(_app_data): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, AuthenticationError> {
    debug!("Authenticating...");
    let access_token = get_token(cookie_jar, &req)?;
    let access_token_details = handle_decode(access_token)?;

    trace!("Claims: {:?}", access_token_details.claims);

    // Can use app_state or redis for further process

    // convert to Identity for future-proof
    let identity = Identity {
        user_id: Uuid::from_str(access_token_details.claims.sub.as_str())
            .map_err(|e| {
                error!("Error mapping user sub to uuid: {:?}", e);
                AuthenticationError::SerializationError
            })?,
        organisation_ids: access_token_details.claims.org_access,
    };

    req.extensions_mut().insert(identity);
    Ok(next.run(req).await)
}

fn get_token(cookie_jar: CookieJar, req: &Request<Body>) -> Result<String, AuthenticationError> {
    trace!("Attempting token from cookie...");
    cookie_jar
        .get("access_token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            trace!("Unable to get token from cookie, attempting from Header Authorization...");
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        })
        .ok_or(AuthenticationError::InvalidToken)
}

pub fn handle_encode_for_user(user: &SwiftUser, organisations: Vec<Uuid>) -> Result<(Claims, String), AuthenticationError> {
    debug!("Creating access token for user");
    let now: DateTime<Utc> = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + *JWT_EXP).timestamp() as usize;

    let claims = Claims {
        jti: Uuid::new_v4().to_string(),
        sub: user.id.to_string(),
        iss: JWT_ISS.clone(),
        aud: vec![JWT_AUD.clone(),],
        iat,
        exp,
        org_access: organisations,
    };

    let access_token = encode(
        &Header::default(),
        &claims,
        &KEYS.encoding
    ).map_err(|e| {
        error!("Unable to create access token {:?}", e);
        AuthenticationError::SerializationError
    })?;
    debug!("Successfully created access token for user");
    Ok((claims, access_token))
}

fn handle_decode(access_token: String) -> Result<TokenData<Claims>, AuthenticationError> {
    trace!("Decoding access token");

    let decoding_result = decode::<Claims>(access_token.as_str(), &(*KEYS).decoding, &*VALIDATION);

    match &decoding_result {
        Ok(_) => debug!("Successfully finished decoding access token."),
        Err(e) => match *e.kind() {
            ErrorKind::InvalidToken => debug!("Invalid token: {}", e),
            ErrorKind::ExpiredSignature => debug!("Expired token: {}", e),
            _ => error!("JWT decoding error: {}", e),
        },
    }

    decoding_result.map_err(|e| match *e.kind() {
        ErrorKind::InvalidToken => AuthenticationError::InvalidToken,
        ErrorKind::ExpiredSignature => AuthenticationError::ExpiredToken,
        _ => AuthenticationError::InvalidToken,
    })
}