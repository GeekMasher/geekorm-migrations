use anyhow::Result;
use geekorm::{ConnectionManager, GeekConnector};

use models::{USERS, Users};

mod cli;
mod models;

#[tokio::main]
async fn main() -> Result<()> {
    cli::init()?;

    // Initialize a database in a file
    let db = ConnectionManager::path("/tmp/geekorm-migration.sqlite").await?;
    let connection = db.acquire().await;

    log::info!("Initializing database...");
    if let Err(err) = db::init(&connection).await {
        log::error!("Error initializing database...");
        log::error!("{}", err);
        return Err(err.into());
    }
    log::info!("Database initialized");

    log::info!("Creating users...");
    for user in USERS {
        match Users::fetch_by_username(&connection, user).await {
            Ok(user) => {
                log::info!(" > User already exists: {}", user.username);
                continue;
            }
            Err(_) => {
                log::info!(" > Creating user: {}", user);
                let mut user = Users::new(user, "password");
                user.fetch_or_create(&connection).await?;
            }
        }
    }
    log::info!("Users created!");

    let total = Users::total(&connection).await?;
    debug_assert_eq!(total, USERS.len() as i64);
    log::info!("Total users: {}", total);

    Ok(())
}
