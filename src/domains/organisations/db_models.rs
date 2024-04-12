use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::schema;

#[derive(Insertable, Queryable, Debug, AsChangeset, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = schema::organisation)]
pub struct Organisation {
    pub id: Uuid,
    pub owner: Uuid,
    pub name: String,
    pub is_archived: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, AsChangeset, Serialize)]
#[diesel(table_name = schema::organisation)]
pub struct PutOrganisation {
    pub owner: Option<Uuid>,
    pub name: String,
    pub updated_at: NaiveDateTime,
}