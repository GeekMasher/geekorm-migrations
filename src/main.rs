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
