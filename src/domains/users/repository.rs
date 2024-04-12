use diesel::prelude::*;

use bb8::{Pool};
use diesel_async::{ AsyncPgConnection, RunQueryDsl};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use uuid::Uuid;
use crate::common::errors::db_error::DbError;

use crate::common::repository::BaseRepository;
use crate::domains::users::db_models::{PutSwiftUser, SwiftUser, SwiftUserOrganisation};
use crate::common::schema::{organisation, swift_user, swift_user_accessible_organisation};

#[derive(Clone)]
pub struct UserRepository {
    pool: Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
}

impl UserRepository {
    pub fn new(pool: Pool<AsyncDieselConnectionManager<AsyncPgConnection>>) -> Self {
        UserRepository { pool }
    }

    pub async fn insert(&self, user: &SwiftUser) -> Result<usize, DbError> {
        let mut conn = self.conn().await?;

        diesel::insert_into(swift_user::table)
            .values(user)
            .execute(&mut conn)
            .await
            .map_err(DbError::from)
    }

    pub async fn find_by_id_and_organisation_id(&self, id: &Uuid, organisation_id: &Uuid) -> Result<Option<SwiftUser>, DbError> {
        let mut conn = self.conn().await?;
        swift_user::table
            .inner_join(swift_user_accessible_organisation::table.on(swift_user_accessible_organisation::swift_user_id.eq(swift_user::id)))
            .filter(swift_user_accessible_organisation::organisation_id.eq(organisation_id))
            .filter(swift_user::id.eq(id))
            .select(swift_user::all_columns)
            .get_result(&mut conn)
            .await
            .optional()
            .map_err(DbError::from)
    }

    pub async fn find_by_email(&self, email: &String) -> Result<Option<SwiftUser>, DbError> {
        let mut conn = self.conn().await?;
        swift_user::table.filter(swift_user::email.eq(email))
            .get_result(&mut conn)
            .await
            .optional()
            .map_err(DbError::from)
    }

    pub async fn update_by_id(&self, id: &Uuid, user: PutSwiftUser) -> Result<usize, DbError> {
        let mut conn = self.conn().await?;
        diesel::update(swift_user::table.find(id))
            .set(&user)
            .execute(&mut conn)
            .await
            .map_err(DbError::from)
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<usize, DbError> {
        let mut conn = self.conn().await?;
        diesel::delete(swift_user::table.filter(swift_user::id.eq(id)))
            .execute(&mut conn)
            .await
            .map_err(DbError::from)
    }

    pub async fn insert_user_accessible_organisation(&self,
                                                     swift_user_organisation: SwiftUserOrganisation) -> Result<usize, DbError> {
        let mut conn = self.conn().await?;

        self.insert_user_accessible_organisation_with_conn(&mut conn, swift_user_organisation).await
    }

    pub async fn insert_user_accessible_organisation_with_conn(&self,
                                                               conn: &mut AsyncPgConnection,
                                                               swift_user_organisation: SwiftUserOrganisation) -> Result<usize, DbError> {
        diesel::insert_into(swift_user_accessible_organisation::table)
            .values(swift_user_organisation)
            .execute(conn)
            .await
            .map_err(DbError::from)
    }

    pub async fn find_all_by_organisation_id(&self, organisation_id: &Uuid) -> Result<Vec<SwiftUser>, DbError> {
        let mut conn = self.conn().await?;

        organisation::table
            .inner_join(swift_user_accessible_organisation::table.on(swift_user_accessible_organisation::organisation_id.eq(organisation::id)))
            .filter(organisation::id.eq(organisation_id))
            .inner_join(swift_user::table.on(swift_user::id.eq(swift_user_accessible_organisation::swift_user_id)))
            .select(swift_user::all_columns)
            .get_results(&mut conn)
            .await
            .map_err(DbError::from)
    }
}

impl BaseRepository for UserRepository {
    fn pool(&self) -> &Pool<AsyncDieselConnectionManager<AsyncPgConnection>> {
        &self.pool
    }
}