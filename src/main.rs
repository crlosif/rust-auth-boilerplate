#[macro_use] extern crate rocket;

mod models;
mod migrations;
mod routes;
mod auth;
mod errors;

use rocket_db_pools::Database;
use sqlx;
use rocket_cors::CorsOptions;

use routes::auth as auth_routes;

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
    
    // Configure CORS
    let cors = CorsOptions::default()
        .allowed_origins(rocket_cors::AllowedOrigins::all())
        .allowed_methods(
            vec![rocket::http::Method::Get, rocket::http::Method::Post]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allowed_headers(rocket_cors::AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Content-Type",
        ]))
        .allow_credentials(true)
        .to_cors()
        .expect("Failed to create CORS fairing");

    let _rocket = rocket::custom(figment)
        .attach(Postgres::init())
        .attach(cors)
        .mount("/", routes![index])
        .mount("/api/auth", routes![
            auth_routes::register,
            auth_routes::login,
            auth_routes::forgot_password,
            auth_routes::reset_password
        ])
        .launch()
        .await?;
    
    Ok(())
}
