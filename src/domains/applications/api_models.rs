use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domains::applications::db_models::{Application, PutApplication};

#[derive(Debug, Deserialize, Serialize)]
pub struct ApplicationCreateRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApplicationPutRequest {
    pub name: String,
    pub description: Option<String>,
}

impl ApplicationCreateRequest {
    pub fn into_with_organisation(self, organisation_id: Uuid) -> Application {
        Application {
            id: Uuid::now_v7(),
            organisation_id,
            name: self.name,
            description: self.description,
            created_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ApplicationResponse {
    pub id: Uuid,
    pub organisation_id: Uuid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub created_at: NaiveDateTime
}

impl Into<ApplicationResponse> for Application {
    fn into(self) -> ApplicationResponse {
        ApplicationResponse {
            id: self.id,
            organisation_id: self.organisation_id,
            name: self.name,
            description: self.description,
            created_at: self.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ApplicationsResponse {
    pub data: Vec<ApplicationResponse>
}

impl Into<PutApplication> for ApplicationPutRequest {
    fn into(self) -> PutApplication {
        PutApplication {
            name: self.name,
            description: self.description,
        }
    }
}