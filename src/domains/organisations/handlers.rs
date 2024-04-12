use axum::extract::{Path, State};
use axum::{Json, Router};
use axum::routing::get;
use http::StatusCode;
use uuid::Uuid;
use crate::config::AppState;

use crate::common::errors::application_error::ApplicationError;
use crate::common::extract::request::SwiftJson;
use crate::common::models::models::Identity;
use crate::domains::organisations::api_models::{OrganisationCreateRequest, OrganisationPutRequest, OrganisationResponse, OrganisationsResponse};
use crate::domains::organisations::db_models::Organisation;
use crate::domains::organisations::services::OrganisationService;
use crate::domains::users::api_models::UsersResponse;
use crate::domains::users::services::UserService;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/organisations", get(fetch_organisations).post(create_organisation))
        .route("/organisations/:organisation_id", get(fetch_organisation).put(update_organisation).delete(delete_organisation))
        .route("/organisations/:organisation_id/users", get(fetch_organisation_users)) // todo move to users and read organisation via header
}

async fn create_organisation(
    identity: Identity,
    State(organisation_service): State<OrganisationService>,
    SwiftJson(organisation_request): SwiftJson<OrganisationCreateRequest>,
) -> Result<(StatusCode, Json<OrganisationResponse>), ApplicationError> {
    // TODO check plan if they can create more than 1 organisation, free user should only able to create 1
    let organisations = organisation_service.find_by_owner(identity.user_id).await?;
    if organisations.capacity() > 1 {
        return Err(ApplicationError::LimitReached("Only 1 owned organisation per user allowed for this plan.".to_string()).into());
    }

    organisation_service.create_by_user(organisation_request.into_with_owner(identity.user_id.clone()), identity)
        .await
        .map(Organisation::into)
        .map(|response| (StatusCode::CREATED, Json(response)))
}

async fn fetch_organisations(
    identity: Identity,
    State(organisation_service): State<OrganisationService>,
) -> Result<Json<OrganisationsResponse>, ApplicationError> {
    organisation_service.find_all(&identity.user_id)
        .await
        .map(|organisations|
            OrganisationsResponse {
                data: organisations.into_iter().map(Organisation::into).collect()
            }
        )
        .map(|response| Json(response))
}

async fn fetch_organisation(
    identity: Identity,
    State(organisation_service): State<OrganisationService>,
    Path(organisation_id): Path<Uuid>,
) -> Result<Json<OrganisationResponse>, ApplicationError> {

    organisation_service.find_by_id_and_accessible_user_id(&identity, &organisation_id)
        .await
        .map(Organisation::into)
        .map(|response| Json(response))
}

async fn fetch_organisation_users(
    identity: Identity,
    State(organisation_service): State<OrganisationService>,
    State(user_service): State<UserService>,
    Path(organisation_id): Path<Uuid>,
) -> Result<Json<UsersResponse>, ApplicationError> {
    let organisation: Organisation = organisation_service.find_by_id_and_accessible_user_id(&identity, &organisation_id).await?;

    user_service.find_all_by_organisation_id(&organisation.id)
        .await
        .map(|users| UsersResponse {
            data: users.into_iter().map(|user| user.into()).collect(),
        })
        .map(|response| Json(response))
}

/*
Only admin is allowed to update organisation
*/
async fn update_organisation(
    identity: Identity,
    Path(organisation_id): Path<Uuid>,
    State(organisation_service): State<OrganisationService>,
    SwiftJson(user_request): SwiftJson<OrganisationPutRequest>,
) -> Result<StatusCode, ApplicationError> {

    match organisation_service.is_admin(&identity.user_id, &organisation_id).await? {
        true => organisation_service.update_by_id(organisation_id, user_request)
            .await
            .map(|_| StatusCode::NO_CONTENT),
        false => Err(ApplicationError::Forbidden)
    }
}

/*
Only admin is allowed to delete organisation
*/
async fn delete_organisation(
    identity: Identity,
    Path(organisation_id): Path<Uuid>,
    State(organisation_service): State<OrganisationService>,
) -> Result<StatusCode, ApplicationError> {

    match organisation_service.is_admin(&identity.user_id, &organisation_id).await? {
        true => organisation_service.delete_by_id(organisation_id)
            .await
            .map(|_| StatusCode::NO_CONTENT),
        false => Err(ApplicationError::Unauthorized)
    }
}