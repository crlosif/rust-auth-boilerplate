use rocket::serde::json::{Json, Value, json};
use rocket::http::Status;
use rocket::response::status;
use rocket_db_pools::Connection;

use crate::models::user::{User, NewUser, LoginUser};
use crate::models::password_reset::{RequestPasswordReset, ResetPassword, PasswordResetToken};
use crate::Postgres;
use crate::auth::jwt::JwtService;
use crate::auth::guard::AuthenticatedUser;
use chrono::{Duration, Utc};

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
            // Generate JWT token
            let token = match JwtService::generate_token(user.id.to_string()) {
                Ok(t) => t,
                Err(_) => {
                    return Err(status::Custom(
                        Status::InternalServerError,
                        Json(json!({
                            "error": "Failed to generate token"
                        })),
                    ));
                }
            };

            Ok(status::Custom(
                Status::Ok,
                Json(json!({
                    "message": "Login successful",
                    "token": token,
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

/// Request password reset - generates a reset token
#[post("/forgot-password", data = "<request>")]
pub async fn forgot_password(
    mut db: Connection<Postgres>,
    request: Json<RequestPasswordReset>,
) -> Result<status::Custom<Json<Value>>, status::Custom<Json<Value>>> {
    // Find user by email
    let result = sqlx::query_as::<_, User>(
        "SELECT id, email, password_hash, created_at, updated_at FROM users WHERE email = $1"
    )
    .bind(&request.email)
    .fetch_optional(&mut **db)
    .await;

    // Always return success to prevent email enumeration
    // In production, you would send an email here
    match result {
        Ok(Some(user)) => {
            // Generate reset token
            let reset_token = uuid::Uuid::new_v4().to_string();
            let expires_at = Utc::now() + Duration::hours(1); // Token expires in 1 hour

            // Store reset token in database
            let insert_result = sqlx::query(
                "INSERT INTO password_reset_tokens (user_id, token, expires_at) VALUES ($1, $2, $3)"
            )
            .bind(&user.id)
            .bind(&reset_token)
            .bind(&expires_at)
            .execute(&mut **db)
            .await;

            match insert_result {
                Ok(_) => {
                    // In production, send email with reset link
                    // For now, return token in response (remove this in production!)
                    Ok(status::Custom(
                        Status::Ok,
                        Json(json!({
                            "message": "Password reset token generated. Check your email.",
                            "token": reset_token // Remove this in production!
                        })),
                    ))
                }
                Err(e) => {
                    eprintln!("Database error: {}", e);
                    Ok(status::Custom(
                        Status::Ok,
                        Json(json!({
                            "message": "If the email exists, a password reset token has been sent."
                        })),
                    ))
                }
            }
        }
        Ok(None) | Err(_) => {
            // Return success to prevent email enumeration
            Ok(status::Custom(
                Status::Ok,
                Json(json!({
                    "message": "If the email exists, a password reset token has been sent."
                })),
            ))
        }
    }
}

/// Reset password using token
#[post("/reset-password", data = "<reset>")]
pub async fn reset_password(
    mut db: Connection<Postgres>,
    reset: Json<ResetPassword>,
) -> Result<status::Custom<Json<Value>>, status::Custom<Json<Value>>> {
    // Validate password length
    if reset.new_password.len() < 6 {
        return Err(status::Custom(
            Status::BadRequest,
            Json(json!({
                "error": "Password must be at least 6 characters long"
            })),
        ));
    }

    // Find valid reset token
    let token_result = sqlx::query_as::<_, PasswordResetToken>(
        "SELECT id, user_id, token, expires_at, used, created_at FROM password_reset_tokens WHERE token = $1"
    )
    .bind(&reset.token)
    .fetch_optional(&mut **db)
    .await;

    let reset_token = match token_result {
        Ok(Some(token)) => token,
        Ok(None) => {
            return Err(status::Custom(
                Status::BadRequest,
                Json(json!({
                    "error": "Invalid or expired reset token"
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

    // Check if token is expired
    if reset_token.expires_at < Utc::now() {
        return Err(status::Custom(
            Status::BadRequest,
            Json(json!({
                "error": "Reset token has expired"
            })),
        ));
    }

    // Check if token has already been used
    if reset_token.used {
        return Err(status::Custom(
            Status::BadRequest,
            Json(json!({
                "error": "Reset token has already been used"
            })),
        ));
    }

    // Hash new password
    let password_hash = match User::hash_password(&reset.new_password) {
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

    // Update user password
    let update_result = sqlx::query(
        "UPDATE users SET password_hash = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2"
    )
    .bind(&password_hash)
    .bind(&reset_token.user_id)
    .execute(&mut **db)
    .await;

    match update_result {
        Ok(_) => {
            // Mark token as used
            let _ = sqlx::query(
                "UPDATE password_reset_tokens SET used = TRUE WHERE token = $1"
            )
            .bind(&reset.token)
            .execute(&mut **db)
            .await;

            Ok(status::Custom(
                Status::Ok,
                Json(json!({
                    "message": "Password reset successfully"
                })),
            ))
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            Err(status::Custom(
                Status::InternalServerError,
                Json(json!({
                    "error": "Failed to reset password"
                })),
            ))
        }
    }
}

/// Protected route example - requires authentication
#[get("/me")]
pub async fn get_current_user(
    user: AuthenticatedUser,
    mut db: Connection<Postgres>,
) -> Result<status::Custom<Json<Value>>, status::Custom<Json<Value>>> {
    // Find user by ID from token
    let result = sqlx::query_as::<_, User>(
        "SELECT id, email, password_hash, created_at, updated_at FROM users WHERE id = $1"
    )
    .bind(&user.user_id)
    .fetch_optional(&mut **db)
    .await;

    match result {
        Ok(Some(user_data)) => {
            Ok(status::Custom(
                Status::Ok,
                Json(json!({
                    "user": {
                        "id": user_data.id.to_string(),
                        "email": user_data.email,
                        "created_at": user_data.created_at.to_rfc3339()
                    }
                })),
            ))
        }
        Ok(None) => {
            Err(status::Custom(
                Status::NotFound,
                Json(json!({
                    "error": "User not found"
                })),
            ))
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            Err(status::Custom(
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error occurred"
                })),
            ))
        }
    }
}
