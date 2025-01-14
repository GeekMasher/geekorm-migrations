use anyhow::Result;
use geekorm::GeekConnector;

use self::models::Users;

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
        log::error!("Error initializing database: {}", err);
        return Err(err.into());
    }

    let users = vec![
        "geekmasher",
        "alice",
        "bob",
        "charlie",
        "dave",
        "eve",
        "frank",
    ];

    for user in &users {
        let mut user = Users::new(*user);
        log::debug!("Creating user: {}", user.username);
        user.fetch_or_create(&connection).await?;
    }

    let total = Users::total(&connection).await?;
    debug_assert_eq!(total, users.len() as i64);
    log::info!("Total users: {}", total);

    Ok(())
}
