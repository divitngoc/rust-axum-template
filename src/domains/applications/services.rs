use tracing::{debug, info};
use uuid::Uuid;

use crate::common::errors::application_error::ApplicationError;
use crate::common::errors::application_error::ApplicationError::NotFound;
use crate::domains::applications::api_models::ApplicationPutRequest;
use crate::domains::applications::db_models::Application;
use crate::domains::applications::repository::ApplicationRepository;

#[derive(Clone)]
pub struct ApplicationService {
    application_repository: ApplicationRepository,
}

impl ApplicationService {
    pub fn new(application_repository: ApplicationRepository) -> Self {
        ApplicationService { application_repository }
    }

    pub async fn create(&self, application: Application) -> Result<Application, ApplicationError> {
        let created_application = self.application_repository
            .insert(&application)
            .await
            .map_err(ApplicationError::from);

        info!("Application successfully created");
        created_application
    }

    pub async fn find_all_by_organisation_id(&self, organisation_id: Uuid) -> Result<Vec<Application>, ApplicationError> {
        debug!("Finding applications...");
        self.application_repository
            .find_all_by_organisation_id(&organisation_id)
            .await
            .map_err(ApplicationError::from)
    }

    pub async fn find_by_id_and_organisation_id(&self, application_id: &Uuid, organisation_id: &Uuid) -> Result<Application, ApplicationError> {
        debug!("Finding application by id: {:?}", application_id);
        let application_option = self.application_repository
            .find_by_id_and_organisation_id(application_id, organisation_id)
            .await
            .map_err(ApplicationError::from)?;

        match application_option {
            Some(organisation) => {
                debug!("Application found.");
                Ok(organisation)
            },
            None => {
                debug!("Could not find application.");
                Err(NotFound)
            }
        }
    }

    pub async fn update_by_id_and_organisation_id(&self, id: Uuid, organisation_id: Uuid, organisation_request: ApplicationPutRequest) -> Result<(), ApplicationError> {
        debug!("Updating application by id: {:?}", id);

        let row_updated = self.application_repository
            .update_by_id_and_organisation_id(&id, &organisation_id, organisation_request.into())
            .await
            .map_err(ApplicationError::from)?;

        debug!("Rows updated {:?}", row_updated);
        if row_updated == 0 {
            Err(NotFound)
        } else {
            Ok(())
        }
    }

    pub async fn delete_by_id_and_organisation_id(&self, id: Uuid, organisation_id: Uuid) -> Result<(), ApplicationError> {
        debug!("Deleting application by id: {:?}", id);

        let row_updated = self.application_repository
            .delete_by_id_and_organisation_id(&id, &organisation_id)
            .await
            .map_err(ApplicationError::from)?;

        if row_updated == 0 {
            Err(NotFound)
        } else {
            Ok(())
        }
    }
}