use sqlx::PgPool;

use crate::{error::AppError, types::RawLogRow};

// This contains all DB operations related to raw logs
pub async fn insert_raw_logs(db: &PgPool, rows: &[RawLogRow]) -> Result<(), AppError> {

    todo!()
}