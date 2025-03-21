use crate::services::UserService;
use actix_web::{web, HttpResponse, Responder, get};
use uuid::Uuid;

/// Get the current user's profile
#[get("/profile")]
pub async fn get_profile(
    user_service: web::Data<UserService>,
    user_id: web::ReqData<Uuid>,
) -> impl Responder {
    // Get user ID from request data (set by JWT middleware)
    let user_id = user_id.into_inner();
    
    // Get user profile
    match user_service.get_profile(user_id).await {
        Ok(profile) => HttpResponse::Ok().json(profile),
        Err(_) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        }))
    }
}
