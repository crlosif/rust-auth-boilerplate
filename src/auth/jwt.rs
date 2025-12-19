use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub exp: usize,  // expiration time
    pub iat: usize,  // issued at
}

impl Claims {
    pub fn new(user_id: String) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(24); // Token expires in 24 hours
        
        Claims {
            sub: user_id,
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
        }
    }
}

pub struct JwtService;

impl JwtService {
    /// Generate a JWT token for a user
    pub fn generate_token(user_id: String) -> Result<String, jsonwebtoken::errors::Error> {
        let secret = std::env::var("ROCKET_JWT_SECRET")
            .expect("ROCKET_JWT_SECRET must be set in .env file");
        
        let claims = Claims::new(user_id);
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )?;
        
        Ok(token)
    }

    /// Verify and decode a JWT token
    pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let secret = std::env::var("ROCKET_JWT_SECRET")
            .expect("ROCKET_JWT_SECRET must be set in .env file");
        
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )?;
        
        Ok(token_data.claims)
    }
}
