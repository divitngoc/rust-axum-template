use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domains::users::db_models::SwiftUser;

#[derive(Debug, Deserialize)]
pub struct UserCreateRequest {
    pub email: String,
    pub password: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserPutRequest {
    pub password: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

impl Into<UserResponse> for SwiftUser {
    fn into(self) -> UserResponse {
        UserResponse {
            id: self.id,
            email: self.email,
            first_name: self.first_name,
            last_name: self.last_name,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UsersResponse {
    pub data: Vec<UserResponse>
}