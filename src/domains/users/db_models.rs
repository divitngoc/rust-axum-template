use chrono::{NaiveDateTime, Utc};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::schema;
use crate::domains::users::api_models::{UserCreateRequest, UserPutRequest};

#[derive(Insertable, Queryable, Debug, AsChangeset, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = schema::swift_user)]
pub struct SwiftUser {
    pub id: Uuid,
    pub email: String,
    pub password: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Into<SwiftUser> for UserCreateRequest {
    fn into(self) -> SwiftUser {
        SwiftUser {
            id: Uuid::now_v7(),
            email: self.email,
            password: self.password,
            first_name: self.first_name,
            last_name: self.last_name,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, AsChangeset, Serialize)]
#[diesel(table_name = schema::swift_user)]
pub struct PutSwiftUser {
    pub password: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub updated_at: NaiveDateTime,
}

impl Into<PutSwiftUser> for UserPutRequest {
    fn into(self) -> PutSwiftUser {
        PutSwiftUser {
            password: self.password,
            first_name: self.first_name,
            last_name: None,
            updated_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Insertable, Queryable, Debug, AsChangeset, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = schema::swift_user_accessible_organisation)]
#[diesel(primary_key(swift_user_id, organisation_id))]
pub struct SwiftUserOrganisation {
    pub swift_user_id: Uuid,
    pub organisation_id: Uuid,
    pub role_id: i64
}