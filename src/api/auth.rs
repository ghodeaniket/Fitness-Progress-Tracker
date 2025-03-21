use crate::models::{UserRegisterRequest, UserLoginRequest};
use crate::services::UserService;
use actix_web::{web, HttpResponse, Responder, post};
use validator::Validate;
use utoipa::OpenApi;

/// Register a new user
///
/// Register a new user with email, username, and password
#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = UserRegisterRequest,
    responses(
        (status = 201, description = "User successfully registered", body = UserProfileResponse),
        (status = 400, description = "Invalid request data"),
        (status = 409, description = "User with this email already exists"),
        (status = 500, description = "Internal server error")
    ),
    tag = "auth"
)]
#[post("/register")]
pub async fn register(
    user_service: web::Data<UserService>,
    req: web::Json<UserRegisterRequest>,
) -> impl Responder {
    // Validate request
    if let Err(errors) = req.validate() {
        return HttpResponse::BadRequest().json(errors);
    }
    
    // Register user
    match user_service.register(req.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => {
            // Check for specific error types (e.g., email already exists)
            if e.to_string().contains("already exists") {
                return HttpResponse::Conflict().json(serde_json::json!({
                    "error": e.to_string()
                }));
            }
            
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to register user"
            }))
        }
    }
}

/// Login a user
///
/// Authenticate a user with email and password
#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = UserLoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 500, description = "Internal server error")
    ),
    tag = "auth"
)]
#[post("/login")]
pub async fn login(
    user_service: web::Data<UserService>,
    req: web::Json<UserLoginRequest>,
) -> impl Responder {
    // Attempt to login
    match user_service.login(req.into_inner()).await {
        Ok((user, token)) => HttpResponse::Ok().json(serde_json::json!({
            "user": user,
            "token": token
        })),
        Err(e) => {
            // For security reasons, be vague about specific errors
            if e.to_string().contains("Invalid email or password") {
                return HttpResponse::Unauthorized().json(serde_json::json!({
                    "error": "Invalid email or password"
                }));
            }
            
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to log in"
            }))
        }
    }
}

// Define a new type for login response for Swagger documentation
#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct LoginResponse {
    user: crate::models::UserProfileResponse,
    token: String,
}
