#[macro_use] extern crate rocket;

use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self, Row};

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
        .merge(("databases.postgres.url", database_url));
    
    let _rocket = rocket::custom(figment)
        .attach(Postgres::init())
        .mount("/", routes![index])
        .launch()
        .await?;
    
    Ok(())
}
