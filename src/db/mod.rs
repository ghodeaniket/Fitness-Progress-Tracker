use anyhow::Result;
use log::{info, error};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

/// Database connection pool
pub type DbPool = PgPool;

/// Initialize database connection pool
pub async fn init_db(database_url: &str) -> Result<DbPool> {
    info!("Initializing database connection pool");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await?;
    
    info!("Database connection established");
    
    // Run migrations (we'll implement this later)
    // run_migrations(&pool).await?;
    
    Ok(pool)
}

/// Run database migrations
/// This function will be implemented in the future when we
/// add SQL migrations
async fn run_migrations(_pool: &DbPool) -> Result<()> {
    // We'll implement this later with sqlx migrations
    Ok(())
}

/// Test the database connection
pub async fn test_connection(pool: &DbPool) -> Result<()> {
    match sqlx::query("SELECT 1").execute(pool).await {
        Ok(_) => {
            info!("Database connection test successful");
            Ok(())
        }
        Err(e) => {
            error!("Database connection test failed: {}", e);
            Err(anyhow::anyhow!("Database connection test failed: {}", e))
        }
    }
}
