use axum::extract::{Path, State};
use axum::{Json, Router};
use axum::routing::get;
use http::StatusCode;
use uuid::Uuid;
use crate::config::AppState;
use crate::domains::users::api_models::{UserPutRequest, UserCreateRequest, UserResponse, UsersResponse};
use crate::domains::users::services::UserService;
use crate::common::errors::application_error::ApplicationError;
use crate::common::errors::global_api_error::ApiError;
use crate::common::extract::request::SwiftJson;
use crate::common::models::models::{Identity, OrganisationId};
use crate::common::security;
use crate::domains::organisations::services::OrganisationService;
use crate::domains::users::db_models::SwiftUser;

pub fn routes() -> Router<AppState> {
    return Router::new()
        .route("/users", get(fetch_users).post(create_user))
        .route("/users/:id", get(fetch_user).put(update_user).delete(delete_user))
        .route_layer(axum::middleware::from_fn(security::middleware::inject_organisation_id));
}

// todo "able" to send invite/email to that user
async fn create_user(
    State(user_service): State<UserService>,
    State(organisation_service): State<OrganisationService>,
    identity: Identity,
    organisation_id: OrganisationId,
    SwiftJson(user_request): SwiftJson<UserCreateRequest>,
) -> Result<(StatusCode, Json<UserResponse>), ApiError> {
    if !organisation_service.is_admin(&identity.user_id, &organisation_id.0).await? {
        return Err(ApplicationError::Forbidden.into());
    }

    user_service.create_with_organisation_id(user_request, organisation_id.0)
        .await
        .map(SwiftUser::into)
        .map(|created_user| (StatusCode::CREATED, Json(created_user)))
}

async fn fetch_users(
    State(user_service): State<UserService>,
    organisation_id: OrganisationId,
) -> Result<Json<UsersResponse>, ApplicationError> {
    user_service.find_all_by_organisation_id(&organisation_id.0)
        .await
        .map(|swift_users| {
            UsersResponse {
                data: swift_users.into_iter().map(SwiftUser::into).collect()
            }
        })
        .map(|users_response| Json(users_response))
}

async fn fetch_user(
    State(user_service): State<UserService>,
    organisation_id: OrganisationId,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, ApplicationError> {
    user_service.find_by_id_and_organisation_id(&id, &organisation_id.0)
        .await
        .map(SwiftUser::into)
        .map(|response| Json(response))
}

async fn update_user(
    Path(id): Path<Uuid>,
    State(organisation_service): State<OrganisationService>,
    identity: Identity,
    organisation_id: OrganisationId,
    State(user_service): State<UserService>,
    SwiftJson(user_request): SwiftJson<UserPutRequest>,
) -> Result<StatusCode, ApplicationError> {
    if !organisation_service.is_admin(&identity.user_id, &organisation_id.0).await? {
        return Err(ApplicationError::Forbidden.into());
    }

    user_service.update_by_id(id, user_request)
        .await
        .map(|_| StatusCode::NO_CONTENT)
}

async fn delete_user(
    Path(id): Path<Uuid>,
    State(organisation_service): State<OrganisationService>,
    identity: Identity,
    organisation_id: OrganisationId,
    State(user_service): State<UserService>,
) -> Result<StatusCode, ApplicationError> {
    if !organisation_service.is_admin(&identity.user_id, &organisation_id.0).await? {
        return Err(ApplicationError::Forbidden.into());
    }

    user_service.delete_by_id(id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
}