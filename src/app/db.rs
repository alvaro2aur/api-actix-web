use sea_orm::{Database, DbErr, DatabaseConnection};

pub async fn init_pool(schema: String) -> Result<DatabaseConnection, DbErr> {
    let config = crate::app::config::Config::from_env(schema);
    let db = Database::connect(&config.database_url).await?;
    println!("Starting database with schema: {}", config.database_url);
    Ok(db)
}