#[macro_use] extern crate rocket;

mod models;
mod migrations;
mod routes;

use rocket_db_pools::Database;
use sqlx;

use routes::auth;

#[derive(Database)]
#[database("postgres")]
struct Postgres(sqlx::PgPool);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Load environment variables from .env file BEFORE Rocket reads config
    dotenv::dotenv().ok();
    
    // Get DATABASE_URL from environment
    let database_url = std::env::var("ROCKET_DATABASE_URL")
        .expect("ROCKET_DATABASE_URL must be set in .env file");
    
    // Configure Rocket with the database URL from .env
    let figment = rocket::Config::figment()
        .merge(("databases.postgres.url", database_url.clone()));
    
    // Run migrations before starting the server
    let pool = sqlx::PgPool::connect(&database_url).await
        .expect("Failed to connect to database");
    migrations::run_migrations(&pool).await
        .expect("Failed to run migrations");
    
    let _rocket = rocket::custom(figment)
        .attach(Postgres::init())
        .mount("/", routes![index])
        .mount("/api/auth", routes![auth::register, auth::login])
        .launch()
        .await?;
    
    Ok(())
}
