use tracing::debug;
use uuid::Uuid;
use crate::domains::users::api_models::{UserPutRequest, UserCreateRequest};
use crate::domains::users::db_models::{SwiftUser, SwiftUserOrganisation};
use crate::domains::users::repository::UserRepository;
use crate::common::errors::api_error_response::ErrorMessage;
use crate::common::errors::application_error::ApplicationError;
use crate::common::errors::application_error::ApplicationError::NotFound;
use crate::common::errors::request_error::RequestError::ValidationError;
use crate::common::errors::global_api_error::ApiError;

#[derive(Clone)]
pub struct UserService {
    user_repository: UserRepository
}

impl UserService {
    pub fn new(user_repository: UserRepository) -> Self {
        UserService { user_repository }
    }

    pub async fn find_all_by_organisation_id(&self, organisation_id: &Uuid) -> Result<Vec<SwiftUser>, ApplicationError> {
        debug!("Finding all users from organisation id: {:?}", organisation_id);

        self.user_repository
            .find_all_by_organisation_id(organisation_id)
            .await
            .map_err(ApplicationError::from)
    }

    pub async fn create(&self, user_request: UserCreateRequest) -> Result<SwiftUser, ApiError> {
        self.create_internal(user_request, None).await
    }

    pub async fn create_with_organisation_id(&self, user_request: UserCreateRequest, organisation_id: Uuid) -> Result<SwiftUser, ApiError> {
        self.create_internal(user_request, Some(organisation_id)).await
    }
    
    async fn create_internal(&self, user_request: UserCreateRequest, organisation_id: Option<Uuid>) -> Result<SwiftUser, ApiError> {
        if !validator::ValidateEmail::validate_email(&user_request.email) {
            return Err(ValidationError(vec![ErrorMessage { field: Some("email".to_string()), message: "Email address is not valid".to_string() }]).into())
        }

        let user: SwiftUser = user_request.into();

        debug!("Creating user: {:?}", user);
        self.user_repository
            .insert(&user)
            .await
            .map_err(ApplicationError::from)?;

        if let Some(organisation_id) = organisation_id {
            self.user_repository.insert_user_accessible_organisation(
                SwiftUserOrganisation {
                    swift_user_id: user.id,
                    organisation_id,
                    role_id: 2, // todo make user default constant
                }
            ).await.map_err(ApplicationError::from)?;
        }

        debug!("User successfully created");
        Ok(user)
    }
    
    pub async fn find_by_id_and_organisation_id(&self, id: &Uuid, organisation_id: &Uuid) -> Result<SwiftUser, ApplicationError> {
        debug!("Finding user by id: {:?}", id);
        let user_option = self.user_repository
            .find_by_id_and_organisation_id(id, organisation_id)
            .await
            .map_err(ApplicationError::from)?;

        match user_option {
            Some(user) => {
                debug!("User found.");
                Ok(user)
            },
            None => {
                debug!("Could not find user.");
                Err(NotFound)
            }
        }
    }

    pub async fn find_by_email(&self, email: &String) -> Result<SwiftUser, ApiError> {
        debug!("Finding user by email: {:?}", email);
        if !validator::ValidateEmail::validate_email(&email) {
            return Err(ValidationError(vec![ErrorMessage { field: Some("email".to_string()), message: "Email address is not valid".to_string() }]).into());
        }

        let user_option = self.user_repository
            .find_by_email(email)
            .await
            .map_err(ApplicationError::from)?;

        match user_option {
            Some(user) => {
                debug!("User found.");
                Ok(user)
            },
            None => {
                debug!("Could not find user.");
                Err(NotFound.into())
            }
        }
    }

    pub async fn update_by_id(&self, id: Uuid, user_request: UserPutRequest) -> Result<(), ApplicationError> {
        debug!("Updating user by id: {:?}", id);

        let row_updated = self.user_repository
            .update_by_id(&id, user_request.into())
            .await
            .map_err(ApplicationError::from)?;

        debug!("Rows updated {:?}", row_updated);
        if row_updated == 0 {
            Err(NotFound)
        } else {
            Ok(())
        }
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<(), ApplicationError> {
        debug!("Deleting user by id: {:?}", id);

        let row_updated = self.user_repository
            .delete_by_id(id)
            .await
            .map_err(ApplicationError::from)?;

        if row_updated == 0 {
            Err(NotFound)
        } else {
            Ok(())
        }
    }
}