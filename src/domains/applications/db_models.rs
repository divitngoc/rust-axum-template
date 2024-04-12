use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::schema;

#[derive(Insertable, Queryable, Debug, AsChangeset, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = schema::application)]
pub struct Application {
    pub id: Uuid,
    pub organisation_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, AsChangeset, Serialize)]
#[diesel(table_name = schema::application)]
pub struct PutApplication {
    pub name: String,
    pub description: Option<String>,
}