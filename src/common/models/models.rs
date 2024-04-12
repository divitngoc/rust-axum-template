use axum::async_trait;
use axum::extract::FromRequestParts;
use http::request::Parts;
use uuid::Uuid;
use crate::common::errors::application_error::ApplicationError;
use crate::common::errors::request_error::RequestError;
use crate::common::utils::constants::ORGANISATION_ID_HEADER;

#[derive(Debug, Clone)]
pub struct Identity {
    pub user_id: Uuid,
    pub organisation_ids: Vec<Uuid>,
}

#[derive(Debug, Clone)]
pub struct OrganisationId(pub Uuid);

#[async_trait]
impl<S> FromRequestParts<S> for Identity
    where
        S: Send + Sync,
{
    type Rejection = ApplicationError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        match parts.extensions.get::<Identity>() {
            Some(identity) => Ok(identity.clone()),
            None => Err(ApplicationError::Unauthorized),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for OrganisationId
    where
        S: Send + Sync,
{
    type Rejection = RequestError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        match parts.extensions.get::<OrganisationId>() {
            Some(organisation_id) => Ok(organisation_id.clone()),
            None => Err(RequestError::HeaderNotFound(ORGANISATION_ID_HEADER.to_string()))
        }
    }
}