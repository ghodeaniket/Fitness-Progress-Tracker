use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;

/// User model that maps to the users table in the database
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    #[serde(skip_serializing)]
    #[schema(hidden = true)]
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// User registration request
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UserRegisterRequest {
    #[validate(email)]
    #[schema(example = "user@example.com")]
    pub email: String,
    
    #[validate(length(min = 3, max = 50))]
    #[schema(example = "johndoe")]
    pub username: String,
    
    #[validate(length(min = 8))]
    #[schema(example = "password123")]
    pub password: String,
    
    #[schema(example = "John")]
    pub first_name: Option<String>,
    
    #[schema(example = "Doe")]
    pub last_name: Option<String>,
}

/// User login request
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UserLoginRequest {
    #[schema(example = "user@example.com")]
    pub email: String,
    
    #[schema(example = "password123")]
    pub password: String,
}

/// User profile response
#[derive(Debug, Serialize, ToSchema)]
pub struct UserProfileResponse {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserProfileResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            created_at: user.created_at,
        }
    }
}

/// Claims for JWT token
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
}
