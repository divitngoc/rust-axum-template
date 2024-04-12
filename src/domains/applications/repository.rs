use diesel::prelude::*;

use bb8::{Pool};
use diesel_async::{ AsyncPgConnection, RunQueryDsl};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use uuid::Uuid;
use crate::common::errors::db_error::DbError;
use crate::common::schema::application;
use crate::common::repository::BaseRepository;
use crate::domains::applications::db_models::{Application, PutApplication};

#[derive(Clone)]
pub struct ApplicationRepository {
    pool: Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
}

impl ApplicationRepository {
    pub fn new(pool: Pool<AsyncDieselConnectionManager<AsyncPgConnection>>) -> Self {
        ApplicationRepository { pool }
    }

    pub async fn insert(&self, application: &Application) -> Result<Application, DbError> {
        let mut conn = self.conn().await?;

        self.insert_with_conn(&mut conn, application).await
    }

    pub async fn insert_with_conn(&self,
                                  conn: &mut AsyncPgConnection,
                                  application: &Application
    ) -> Result<Application, DbError> {
        diesel::insert_into(application::table)
            .values(application)
            .get_result(conn)
            .await
            .map_err(DbError::from)
    }

    pub async fn find_all_by_organisation_id(&self, organisation_id: &Uuid) -> Result<Vec<Application>, DbError> {
        let mut conn = self.conn().await?;
        application::table
            .filter(application::organisation_id.eq(organisation_id))
            .get_results(&mut conn)
            .await
            .map_err(DbError::from)
    }

    pub async fn find_by_id_and_organisation_id(&self, application_id: &Uuid, organisation_id: &Uuid) -> Result<Option<Application>, DbError> {
        let mut conn = self.conn().await?;
        application::table
            .filter(application::id.eq(application_id))
            .filter(application::organisation_id.eq(organisation_id))
            .get_result(&mut conn)
            .await
            .optional()
            .map_err(DbError::from)
    }

    pub async fn update_by_id_and_organisation_id(&self, id: &Uuid, organisation_id: &Uuid, application: PutApplication) -> Result<usize, DbError> {
        let mut conn = self.conn().await?;
        diesel::update(application::table.find(id).filter(application::organisation_id.eq(organisation_id)))
            .set(&application)
            .execute(&mut conn)
            .await
            .map_err(DbError::from)
    }

    pub async fn delete_by_id_and_organisation_id(&self, id: &Uuid, organisation_id: &Uuid) -> Result<usize, DbError> {
        let mut conn = self.conn().await?;
        diesel::delete(application::table.filter(application::id.eq(id)).filter(application::organisation_id.eq(organisation_id)))
            .execute(&mut conn)
            .await
            .map_err(DbError::from)
    }
}

impl BaseRepository for ApplicationRepository {
    fn pool(&self) -> &Pool<AsyncDieselConnectionManager<AsyncPgConnection>> {
        &self.pool
    }
}