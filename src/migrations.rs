use sqlx::PgPool;

/// Run database migrations
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Execute each SQL statement separately
    // SQLx cannot execute multiple statements in a single query
    
    // Create users table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            email VARCHAR(255) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create index on email
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_users_email ON users(email)"
    )
    .execute(pool)
    .await?;
    
    println!("✓ Database migrations completed successfully");
    Ok(())
}
