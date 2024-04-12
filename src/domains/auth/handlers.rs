use axum::extract::State;
use axum::{Json, Router};
use axum::routing::post;
use http::{header, StatusCode};
use argon2::{self, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use axum::response::{IntoResponse, Response};
use tracing::{debug, error};
use crate::config::AppState;
use crate::common::errors::application_error::ApplicationError;
use crate::common::errors::global_api_error::ApiError;
use crate::common::extract::request::SwiftJson;
use crate::common::security::jwt;
use crate::common::security::jwt::{Claims};
use crate::common::utils;
use crate::config::app_env::JWT_AUD;
use crate::domains::auth::api_models::{LoginRequest, LoginResponse};
use crate::domains::organisations::services::OrganisationService;
use crate::domains::users::api_models::UserCreateRequest;
use crate::domains::users::db_models::SwiftUser;
use crate::domains::users::services::UserService;

pub fn routes() -> Router<AppState> {
    return Router::new()
        .route("/auth/signup", post(auth_signup_user))
        .route("/auth/login", post(auth_login_user))
}

async fn auth_signup_user(
    State(user_service): State<UserService>,
    SwiftJson(signup_request): SwiftJson<LoginRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // TODO validate password
    let user_request = convert_to_user_request(signup_request)?;

    let user: SwiftUser = user_service.create(user_request).await?;

    let (claims, access_token): (Claims, String) = jwt::handle_encode_for_user(&user, vec![])?;

    let mut response = (
        StatusCode::CREATED, 
        Json(LoginResponse { 
            access_token: access_token.clone(), 
        })
    ).into_response();

    set_cookie(&claims, access_token, &mut response);

    Ok(response)
}

async fn auth_login_user(
    State(user_service): State<UserService>,
    State(organisation_service): State<OrganisationService>,
    SwiftJson(login_request): SwiftJson<LoginRequest>,
) -> Result<impl IntoResponse, ApiError> {

    let user: SwiftUser = user_service.find_by_email(&login_request.email).await?;

    debug!("Comparing password...");
    user.password.as_ref()
        .ok_or(ApplicationError::LoginError) // Maybe add passwordDoesNotExist
        .and_then(|p| PasswordHash::new(p).map_err(|_| ApplicationError::InternalServerError))
        .and_then(|user_password| Argon2::default()
            .verify_password(&login_request.password.as_bytes(), &user_password)
            .map_err(|_| ApplicationError::LoginError))?;
    debug!("Finished comparing password.");

    let organisations = organisation_service.find_all(&user.id).await?;
    let (claims, access_token): (Claims, String) = jwt::handle_encode_for_user(&user, organisations.into_iter().map(|o| o.id).collect())?;
    
    let mut response = (StatusCode::OK, Json(LoginResponse {
        access_token: access_token.clone(),
    })).into_response();

    set_cookie(&claims, access_token, &mut response);

    Ok(response)
}

fn set_cookie(claims: &Claims, access_token: String, response: &mut Response) {
    let cookie = utils::cookie::build_cookie(claims, "access_token".to_string(), access_token, JWT_AUD.clone());
    response.headers_mut().insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
}

fn convert_to_user_request(signup_request: LoginRequest) -> Result<UserCreateRequest, ApiError> {
    let salt = SaltString::generate(&mut OsRng);

    Ok(UserCreateRequest {
        email: signup_request.email,
        password:
        Some(Argon2::default()
            .hash_password(signup_request.password.as_bytes(), &salt)
            .map_err(|e| {
                error!("Failed to hash password: {:?}", e);
                ApplicationError::InternalServerError
            })
            .map(|password_hash| password_hash.to_string())?
        ),
        first_name: "John".to_string(),
        last_name: Some("Doe".to_string()),
    })
}