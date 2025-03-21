use crate::services::UserService;
use actix_web::{web, HttpResponse, Responder, get};
use uuid::Uuid;
use utoipa::OpenApi;

/// Get the current user's profile
///
/// Retrieve the profile information for the authenticated user
#[utoipa::path(
    get,
    path = "/users/profile",
    responses(
        (status = 200, description = "User profile retrieved successfully", body = UserProfileResponse),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    tag = "users",
    security(
        ("jwt_auth" = [])
    )
)]
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
