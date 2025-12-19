use rocket::request::{FromRequest, Request, Outcome};
use rocket::http::Status;
use crate::auth::jwt::JwtService;

/// Request guard for authenticated users
/// Use this in route handlers to protect routes that require authentication
/// 
/// Example:
/// ```rust
/// #[get("/protected")]
/// fn protected_route(user: AuthenticatedUser) -> String {
///     format!("Hello, user {}!", user.user_id)
/// }
/// ```
pub struct AuthenticatedUser {
    pub user_id: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Get the Authorization header
        let auth_header = request.headers().get_one("Authorization");
        
        match auth_header {
            Some(header) => {
                // Check if it starts with "Bearer "
                if !header.starts_with("Bearer ") {
                    return Outcome::Error((Status::Unauthorized, ()));
                }
                
                // Extract the token
                let token = &header[7..]; // Skip "Bearer "
                
                // Verify the token
                match JwtService::verify_token(token) {
                    Ok(claims) => {
                        Outcome::Success(AuthenticatedUser {
                            user_id: claims.sub,
                        })
                    }
                    Err(_) => Outcome::Error((Status::Unauthorized, ())),
                }
            }
            None => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
