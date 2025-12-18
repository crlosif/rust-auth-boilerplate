use rocket::serde::json::{Json, Value, json};
use rocket::http::Status;
use rocket::response::status;
use rocket_db_pools::Connection;

use crate::models::user::{User, NewUser, LoginUser};
use crate::Postgres;

/// Register a new user
#[post("/register", data = "<new_user>")]
pub async fn register(
    mut db: Connection<Postgres>,
    new_user: Json<NewUser>,
) -> Result<status::Custom<Json<Value>>, status::Custom<Json<Value>>> {
    // Validate email format (basic validation)
    if !new_user.email.contains('@') {
        return Err(status::Custom(
            Status::BadRequest,
            Json(json!({
                "error": "Invalid email format"
            })),
        ));
    }

    // Validate password length
    if new_user.password.len() < 6 {
        return Err(status::Custom(
            Status::BadRequest,
            Json(json!({
                "error": "Password must be at least 6 characters long"
            })),
        ));
    }

    // Check if user already exists
    let existing_user = sqlx::query_scalar::<_, Option<uuid::Uuid>>("SELECT id FROM users WHERE email = $1")
        .bind(&new_user.email)
        .fetch_optional(&mut **db)
        .await;

    match existing_user {
        Ok(Some(_)) => {
            return Err(status::Custom(
                Status::Conflict,
                Json(json!({
                    "error": "User with this email already exists"
                })),
            ));
        }
        Ok(None) => {}
        Err(e) => {
            eprintln!("Database error: {}", e);
            return Err(status::Custom(
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error occurred"
                })),
            ));
        }
    }

    // Hash the password
    let password_hash = match User::hash_password(&new_user.password) {
        Ok(hash) => hash,
        Err(_) => {
            return Err(status::Custom(
                Status::InternalServerError,
                Json(json!({
                    "error": "Failed to hash password"
                })),
            ));
        }
    };

    // Insert new user into database
    let result = sqlx::query_as::<_, User>(
        "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id, email, password_hash, created_at, updated_at"
    )
    .bind(&new_user.email)
    .bind(&password_hash)
    .fetch_one(&mut **db)
    .await;

    match result {
        Ok(user) => {

            Ok(status::Custom(
                Status::Created,
                Json(json!({
                    "message": "User registered successfully",
                    "user": {
                        "id": user.id.to_string(),
                        "email": user.email,
                        "created_at": user.created_at.to_rfc3339()
                    }
                })),
            ))
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            Err(status::Custom(
                Status::InternalServerError,
                Json(json!({
                    "error": "Failed to create user"
                })),
            ))
        }
    }
}

/// Login endpoint
#[post("/login", data = "<login_user>")]
pub async fn login(
    mut db: Connection<Postgres>,
    login_user: Json<LoginUser>,
) -> Result<status::Custom<Json<Value>>, status::Custom<Json<Value>>> {
    // Find user by email
    let result = sqlx::query_as::<_, User>(
        "SELECT id, email, password_hash, created_at, updated_at FROM users WHERE email = $1"
    )
    .bind(&login_user.email)
    .fetch_optional(&mut **db)
    .await;

    let user = match result {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err(status::Custom(
                Status::Unauthorized,
                Json(json!({
                    "error": "Invalid email or password"
                })),
            ));
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            return Err(status::Custom(
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error occurred"
                })),
            ));
        }
    };

    // Verify password
    match User::verify_password(&login_user.password, &user.password_hash) {
        Ok(true) => {
            Ok(status::Custom(
                Status::Ok,
                Json(json!({
                    "message": "Login successful",
                    "user": {
                        "id": user.id.to_string(),
                        "email": user.email,
                        "created_at": user.created_at.to_rfc3339()
                    }
                })),
            ))
        }
        Ok(false) => {
            Err(status::Custom(
                Status::Unauthorized,
                Json(json!({
                    "error": "Invalid email or password"
                })),
            ))
        }
        Err(_) => {
            Err(status::Custom(
                Status::InternalServerError,
                Json(json!({
                    "error": "Failed to verify password"
                })),
            ))
        }
    }
}
