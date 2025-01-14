use anyhow::Result;
use geekorm::GeekConnector;

use models::{Users, USERS};

mod cli;
mod models;

#[tokio::main]
async fn main() -> Result<()> {
    cli::init()?;

    // Initialize a database in a file
    let db = libsql::Builder::new_local("/tmp/geekorm.db")
        .build()
        .await?;
    let connection = db.connect()?;

    if let Err(err) = db::init(&connection).await {
        log::error!("Error initializing database...");
        log::error!("{}", err);
        return Err(err.into());
    }

    // Create the users
    for user in USERS {
        let mut user = Users::new(user);
        log::debug!("Creating user: {}", user.username);
        user.fetch_or_create(&connection).await?;
    }

    let total = Users::total(&connection).await?;
    debug_assert_eq!(total, USERS.len() as i64);
    log::info!("Total users: {}", total);

    Ok(())
}
