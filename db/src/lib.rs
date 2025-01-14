#![doc = r" GeekORM Database Migrations"]
#[allow(unused_imports, unused_variables)]
use geekorm::prelude::*;
mod v0_1_0;
pub use v0_1_0::{Database, Migration as LatestMigration};
pub async fn init<'a, T>(connection: &'a T) -> Result<(), geekorm::Error>
where
    T: geekorm::GeekConnection<Connection = T> + 'a,
{
    let latest = &LatestMigration;
    match latest.validate_database(connection, &Database).await {
        Ok(MigrationState::Initialized) => {
            log::info!("Database is initialized");
            LatestMigration::create(connection).await?;
        }
        Ok(MigrationState::UpToDate) => {
            log::info!("Database is up to date");
        }
        Ok(MigrationState::OutOfDate(reason)) => {
            log::error!("Database is out of date. Expected version: {}", reason);
            return Err(geekorm::Error::Unknown);
        }
        Err(err) => {
            log::error!("Error validating database: {}", err);
            return Err(err);
        }
    }
    Ok(())
}

