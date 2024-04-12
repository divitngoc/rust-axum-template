
use axum::async_trait;
use bb8::{Pool, PooledConnection};
use diesel_async::{AsyncPgConnection};
use crate::common::errors::db_error::DbError;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;


#[async_trait]
pub trait BaseRepository {
    fn pool(&self) -> &Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

    async fn conn(&self) -> Result<PooledConnection<AsyncDieselConnectionManager<AsyncPgConnection>>, DbError> {
        self.pool().get().await.map_err(DbError::from)
    }
}