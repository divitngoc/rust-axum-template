use diesel_async::{AsyncConnection};
use tracing::{debug, info};
use uuid::Uuid;

use crate::common::errors::application_error::ApplicationError;
use crate::common::errors::application_error::ApplicationError::NotFound;
use crate::common::models::models::Identity;
use crate::common::repository::BaseRepository;
use crate::domains::organisations::api_models::{OrganisationPutRequest};
use crate::domains::organisations::db_models::Organisation;
use crate::domains::organisations::repository::OrganisationRepository;
use crate::domains::users::db_models::SwiftUserOrganisation;
use crate::domains::users::repository::UserRepository;

#[derive(Clone)]
pub struct OrganisationService {
    organisation_repository: OrganisationRepository,
    user_repository: UserRepository, 
}

impl OrganisationService {
    pub fn new(organisation_repository: OrganisationRepository,  user_repository: UserRepository) -> Self {
        OrganisationService { organisation_repository, user_repository }
    }

    pub async fn create_by_user(&self, organisation: Organisation, identity: Identity) -> Result<Organisation, ApplicationError> {
        let mut conn = self.organisation_repository.conn().await?;

        let created_organisation = conn.transaction(|tx| {
            Box::pin(async move {
                debug!("Creating organisation: {:?}", organisation);
                let result = self.organisation_repository
                    .insert_with_conn(tx, &organisation)
                    .await
                    .map_err(ApplicationError::from);

                debug!("Inserting user accessible organisation...");
                self.user_repository.insert_user_accessible_organisation_with_conn(tx, SwiftUserOrganisation {
                    swift_user_id: identity.user_id.clone(),
                    organisation_id: organisation.id,
                    role_id: 1, // Admin
                })
                .await
                .map_err(ApplicationError::from)?;

                result
            })
        }).await;

        info!("Organisation successfully created");
        created_organisation
    }

    pub async fn find_all(&self, user_id: &Uuid) -> Result<Vec<Organisation>, ApplicationError> {
        debug!("Finding organisations...");
        self.organisation_repository
            .find_all_accessible(user_id)
            .await
            .map_err(ApplicationError::from)
    }

    pub async fn find_by_id_and_accessible_user_id(&self, identity: &Identity, organisation_id: &Uuid) -> Result<Organisation, ApplicationError> {
        debug!("Finding organisation by id: {:?}", organisation_id);
        let organisation_option = self.organisation_repository
            .find_by_accessible_user_id(organisation_id, &identity.user_id)
            .await
            .map_err(ApplicationError::from)?;

        match organisation_option {
            Some(organisation) => {
                debug!("Organisation found.");
                Ok(organisation)
            },
            None => {
                debug!("Could not find organisation.");
                Err(NotFound)
            }
        }
    }

    pub async fn find_by_owner(&self, user_id: Uuid) -> Result<Vec<Organisation>, ApplicationError> {
        self.organisation_repository.find_by_owner(&user_id)
            .await
            .map_err(ApplicationError::from)
    }

    pub async fn update_by_id(&self, id: Uuid, organisation_request: OrganisationPutRequest) -> Result<(), ApplicationError> {
        debug!("Updating organisation by id: {:?}", id);

        let row_updated = self.organisation_repository
            .update_by_id(&id, organisation_request.into())
            .await
            .map_err(ApplicationError::from)?;

        debug!("Rows updated {:?}", row_updated);
        if row_updated == 0 {
            Err(NotFound)
        } else {
            Ok(())
        }
    }

    pub async fn is_admin(&self, user_id: &Uuid, organisation_id: &Uuid) -> Result<bool, ApplicationError> {
        let role_id: Result<Option<i64>, ApplicationError> = self.organisation_repository
            .find_role(organisation_id, user_id)
            .await
            .map_err(ApplicationError::from);

        match role_id {
            Ok(Some(rid)) => Ok(rid == 1),
            Ok(None) => Ok(false),
            Err(e) => Err(e),
        }
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<(), ApplicationError> {
        debug!("Deleting organisation by id: {:?}", id);

        let row_updated = self.organisation_repository
            .delete_by_id(&id)
            .await
            .map_err(ApplicationError::from)?;

        if row_updated == 0 {
            Err(NotFound)
        } else {
            Ok(())
        }
    }
}