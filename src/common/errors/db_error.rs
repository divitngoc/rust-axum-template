use bb8::RunError;
use diesel_async::pooled_connection::PoolError;

use diesel::result::Error as DieselError;

#[derive(Debug)]
pub enum DbError {
    PoolError(RunError<PoolError>),
    DieselError(DieselError),
}

impl From<RunError<PoolError>> for DbError {
    fn from(error: RunError<PoolError>) -> Self {
        DbError::PoolError(error)
    }
}

// Implement the conversion from DieselError to AppError
impl From<DieselError> for DbError {
    fn from(error: DieselError) -> Self {
        DbError::DieselError(error)
    }
}