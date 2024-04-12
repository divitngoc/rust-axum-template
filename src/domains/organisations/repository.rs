use diesel::prelude::*;

use bb8::{Pool};
use diesel_async::{ AsyncPgConnection, RunQueryDsl};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use uuid::Uuid;
use crate::common::errors::db_error::DbError;
use crate::common::schema::{organisation, swift_user_accessible_organisation};
use crate::common::repository::BaseRepository;
use crate::domains::organisations::db_models::{Organisation, PutOrganisation};

#[derive(Clone)]
pub struct OrganisationRepository {
    pool: Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
}

impl OrganisationRepository {
    pub fn new(pool: Pool<AsyncDieselConnectionManager<AsyncPgConnection>>) -> Self {
        OrganisationRepository { pool }
    }

    pub async fn insert_with_conn(&self,
                                  conn: &mut AsyncPgConnection,
                                  organisation: &Organisation
    ) -> Result<Organisation, DbError> {
        diesel::insert_into(organisation::table)
            .values(organisation)
            .get_result(conn)
            .await
            .map_err(DbError::from)
    }

    pub async fn find_all_accessible(&self, user_id: &Uuid) -> Result<Vec<Organisation>, DbError> {
        let mut conn = self.conn().await?;
        organisation::table
            .inner_join(swift_user_accessible_organisation::table.on(organisation::id.eq(swift_user_accessible_organisation::organisation_id)))
            .filter(swift_user_accessible_organisation::swift_user_id.eq(user_id))
            .select(organisation::all_columns)
            .load::<Organisation>(&mut conn)
            .await
            .map_err(DbError::from)
    }

    pub async fn find_by_accessible_user_id(&self, organisation_id: &Uuid, user_id: &Uuid) -> Result<Option<Organisation>, DbError> {
        let mut conn = self.conn().await?;
        organisation::table
            .inner_join(swift_user_accessible_organisation::table.on(organisation::id.eq(swift_user_accessible_organisation::organisation_id)))
            .filter(organisation::id.eq(organisation_id))
            .filter(swift_user_accessible_organisation::swift_user_id.eq(user_id))
            .select(organisation::all_columns)
            .get_result(&mut conn)
            .await
            .optional()
            .map_err(DbError::from)
    }

    pub async fn find_by_owner(&self, id: &Uuid) -> Result<Vec<Organisation>, DbError> {
        let mut conn = self.conn().await?;
        organisation::table.filter(organisation::owner.eq(id))
            .get_results(&mut conn)
            .await
            .map_err(DbError::from)
    }

    pub async fn update_by_id(&self, id: &Uuid, organisation: PutOrganisation) -> Result<usize, DbError> {
        let mut conn = self.conn().await?;
        diesel::update(organisation::table.find(id))
            .set(&organisation)
            .execute(&mut conn)
            .await
            .map_err(DbError::from)
    }

    pub async fn delete_by_id(&self, id: &Uuid) -> Result<usize, DbError> {
        let mut conn = self.conn().await?;
        diesel::delete(organisation::table.filter(organisation::id.eq(id)))
            .execute(&mut conn)
            .await
            .map_err(DbError::from)
    }

    pub async fn find_role(&self, organisation_id: &Uuid, user_id: &Uuid) -> Result<Option<i64>, DbError> {
        let mut conn = self.conn().await?;
        organisation::table
            .inner_join(swift_user_accessible_organisation::table.on(organisation::id.eq(swift_user_accessible_organisation::organisation_id)))
            .filter(organisation::id.eq(organisation_id))
            .filter(swift_user_accessible_organisation::swift_user_id.eq(user_id))
            .select(swift_user_accessible_organisation::columns::role_id)
            .get_result(&mut conn)
            .await
            .optional()
            .map_err(DbError::from)
    }
}

impl BaseRepository for OrganisationRepository {
    fn pool(&self) -> &Pool<AsyncDieselConnectionManager<AsyncPgConnection>> {
        &self.pool
    }
}