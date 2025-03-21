use crate::models::{UserRegisterRequest, UserLoginRequest};
use crate::services::UserService;
use actix_web::{web, HttpResponse, Responder, post};
use validator::Validate;

/// Register a new user
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
