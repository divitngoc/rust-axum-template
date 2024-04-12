use std::env;
use bb8::Pool;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager};

pub async fn establish_connection() -> Pool<AsyncDieselConnectionManager<AsyncPgConnection>> {
    let database_url: String = env::var("DATABASE_URL").unwrap();
    let manager: AsyncDieselConnectionManager<AsyncPgConnection> = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    Pool::builder().build(manager).await.expect("Failed to create pool.")
}