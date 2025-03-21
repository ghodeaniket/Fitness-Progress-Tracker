use crate::db::DbPool;
use crate::models::{User, UserRegisterRequest, UserLoginRequest, UserProfileResponse};
use crate::utils::{hash_password, verify_password, generate_token};
use anyhow::{Result, anyhow};
use chrono::Utc;
use sqlx::query_as;
use uuid::Uuid;

/// Service for handling user-related operations
pub struct UserService {
    db_pool: DbPool,
    jwt_secret: String,
    jwt_expiration: u64,
}

impl UserService {
    /// Create a new UserService instance
    pub fn new(db_pool: DbPool, jwt_secret: String, jwt_expiration: u64) -> Self {
        Self {
            db_pool,
            jwt_secret,
            jwt_expiration,
        }
    }
    
    /// Register a new user
    pub async fn register(&self, req: UserRegisterRequest) -> Result<UserProfileResponse> {
        // Check if user with this email already exists
        let existing_user = sqlx::query!(
            "SELECT id FROM users WHERE email = $1",
            req.email
        )
        .fetch_optional(&self.db_pool)
        .await?;
        
        if existing_user.is_some() {
            return Err(anyhow!("User with this email already exists"));
        }
        
        // Check if username is taken
        let existing_username = sqlx::query!(
            "SELECT id FROM users WHERE username = $1",
            req.username
        )
        .fetch_optional(&self.db_pool)
        .await?;
        
        if existing_username.is_some() {
            return Err(anyhow!("Username is already taken"));
        }
        
        // Hash the password
        let password_hash = hash_password(&req.password)?;
        
        // Create new user
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, email, username, password_hash, first_name, last_name, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, email, username, password_hash, first_name, last_name, created_at, updated_at
            "#,
            Uuid::new_v4(),
            req.email,
            req.username,
            password_hash,
            req.first_name,
            req.last_name,
            Utc::now(),
            Utc::now()
        )
        .fetch_one(&self.db_pool)
        .await?;
        
        Ok(user.into())
    }
    
    /// Login a user
    pub async fn login(&self, req: UserLoginRequest) -> Result<(UserProfileResponse, String)> {
        // Find user by email
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, username, password_hash, first_name, last_name, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
            req.email
        )
        .fetch_optional(&self.db_pool)
        .await?
        .ok_or_else(|| anyhow!("Invalid email or password"))?;
        
        // Verify password
        let is_valid = verify_password(&req.password, &user.password_hash)?;
        
        if !is_valid {
            return Err(anyhow!("Invalid email or password"));
        }
        
        // Generate JWT token
        let token = generate_token(user.id, &self.jwt_secret, self.jwt_expiration)?;
        
        Ok((user.into(), token))
    }
    
    /// Get user profile by ID
    pub async fn get_profile(&self, user_id: Uuid) -> Result<UserProfileResponse> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, username, password_hash, first_name, last_name, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.db_pool)
        .await?
        .ok_or_else(|| anyhow!("User not found"))?;
        
        Ok(user.into())
    }
}
