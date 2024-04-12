use axum::extract::{Path, State};
use axum::{Json, Router};
use axum::routing::get;
use http::{StatusCode};
use uuid::Uuid;
use crate::config::AppState;

use crate::common::errors::application_error::ApplicationError;
use crate::common::errors::global_api_error::ApiError;
use crate::common::extract::request::SwiftJson;
use crate::common::models::models::{OrganisationId};
use crate::common::security;
use crate::domains::applications::api_models::{ApplicationCreateRequest, ApplicationPutRequest, ApplicationResponse, ApplicationsResponse};
use crate::domains::applications::db_models::Application;
use crate::domains::applications::services::ApplicationService;


pub fn routes() -> Router<AppState> {
    return Router::new()
        .route("/applications", get(fetch_applications).post(create_applications))
        .route("/applications/:id", get(fetch_application).put(update_application).delete(delete_application))
        .route_layer(axum::middleware::from_fn(security::middleware::inject_organisation_id))
}

async fn create_applications(
    State(application_service): State<ApplicationService>,
    organisation_id: OrganisationId,
    SwiftJson(application_request): SwiftJson<ApplicationCreateRequest>,
) -> Result<(StatusCode, Json<ApplicationResponse>), ApiError> {

    // TODO check plan if they can create more than 3, free user should only able to create 3
    application_service.create(application_request.into_with_organisation(organisation_id.0))
        .await
        .map(Application::into)
        .map_err(ApiError::from)
        .map(|response| (StatusCode::CREATED, Json(response)))
}

async fn fetch_applications(
    State(application_service): State<ApplicationService>,
    organisation_id: OrganisationId,
) -> Result<Json<ApplicationsResponse>, ApplicationError> {

    application_service.find_all_by_organisation_id(organisation_id.0)
        .await
        .map(|applications|
            ApplicationsResponse {
                data: applications.into_iter().map(Application::into).collect()
            }
        )
        .map(|response| Json(response))
}

async fn fetch_application(
    State(application_service): State<ApplicationService>,
    organisation_id: OrganisationId,
    Path(application_id): Path<Uuid>,
) -> Result<Json<ApplicationResponse>, ApplicationError> {

    application_service.find_by_id_and_organisation_id(&application_id, &organisation_id.0)
        .await
        .map(Application::into)
        .map(|response| Json(response))
}

async fn update_application(
    State(application_service): State<ApplicationService>,
    organisation_id: OrganisationId,
    Path(application_id): Path<Uuid>,
    SwiftJson(application_put_request): SwiftJson<ApplicationPutRequest>,
) -> Result<StatusCode, ApplicationError> {

    application_service.update_by_id_and_organisation_id(application_id, organisation_id.0, application_put_request)
        .await
        .map(|_| StatusCode::NO_CONTENT)
}

/*
Only admin is allowed to delete application
*/
async fn delete_application(
    State(application_service): State<ApplicationService>,
    organisation_id: OrganisationId,
    Path(application_id): Path<Uuid>,
) -> Result<StatusCode, ApplicationError> {

    application_service.delete_by_id_and_organisation_id(application_id, organisation_id.0)
        .await
        .map(|_| StatusCode::NO_CONTENT)
}