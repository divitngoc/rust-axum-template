use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domains::organisations::db_models::{Organisation, PutOrganisation};

#[derive(Debug, Deserialize, Serialize)]
pub struct OrganisationCreateRequest {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrganisationPutRequest {
    pub owner: Option<Uuid>,
    pub name: String,
}

impl OrganisationCreateRequest {
    pub fn into_with_owner(self, owner: Uuid) -> Organisation {
        Organisation {
            id: Uuid::now_v7(),
            owner,
            name: self.name,
            is_archived: false,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct OrganisationResponse {
    pub id: Uuid,
    pub owner: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime
}

impl Into<OrganisationResponse> for Organisation {
    fn into(self) -> OrganisationResponse {
        OrganisationResponse {
            id: self.id,
            owner: self.owner,
            name: self.name,
            created_at: self.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct OrganisationsResponse {
    pub data: Vec<OrganisationResponse>
}

impl Into<PutOrganisation> for OrganisationPutRequest {
    fn into(self) -> PutOrganisation {
        PutOrganisation {
            owner: self.owner,
            name: self.name,
            updated_at: Utc::now().naive_utc(),
        }
    }
}