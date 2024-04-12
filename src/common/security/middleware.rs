use axum::body::Body;
use axum::middleware::Next;
use axum::response::IntoResponse;
use http::Request;
use crate::common::errors::application_error::ApplicationError;
use crate::common::errors::global_api_error::ApiError;

use crate::common::models::models::{Identity, OrganisationId};
use crate::common::utils::constants::ORGANISATION_ID_HEADER;
use crate::common::utils::header_utils;

pub async fn inject_organisation_id(
    mut req: Request<Body>,
    next: Next, ) -> Result<impl IntoResponse, ApiError> {

    if let Some(identity) = req.extensions().get::<Identity>() {
        let organisation_id = header_utils::extract_uuid_from_required_header(req.headers(), &ORGANISATION_ID_HEADER)?;
        if identity.organisation_ids.contains(&organisation_id) {
            req.extensions_mut().insert(OrganisationId(organisation_id));
            Ok(next.run(req).await)
        } else {
            Err(ApplicationError::Unauthorized.into())
        }
    } else {
        Err(ApplicationError::Unauthorized.into())
    }
}