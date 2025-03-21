use crate::models::CreateWorkoutRequest;
use crate::services::WorkoutService;
use actix_web::{web, HttpResponse, Responder, get, post, delete};
use uuid::Uuid;
use validator::Validate;
use utoipa::OpenApi;

/// Create a new workout
///
/// Create a new workout for the authenticated user
#[utoipa::path(
    post,
    path = "/workouts",
    request_body = CreateWorkoutRequest,
    responses(
        (status = 201, description = "Workout created successfully", body = CreateWorkoutResponse),
        (status = 400, description = "Invalid request data"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    tag = "workouts",
    security(
        ("jwt_auth" = [])
    )
)]
#[post("")]
pub async fn create_workout(
    workout_service: web::Data<WorkoutService>,
    user_id: web::ReqData<Uuid>,
    req: web::Json<CreateWorkoutRequest>,
) -> impl Responder {
    // Validate request
    if let Err(errors) = req.validate() {
        return HttpResponse::BadRequest().json(errors);
    }
    
    // Get user ID from request data (set by JWT middleware)
    let user_id = user_id.into_inner();
    
    // Create workout
    match workout_service.create_workout(user_id, req.into_inner()).await {
        Ok(workout_id) => HttpResponse::Created().json(serde_json::json!({
            "id": workout_id
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to create workout: {}", e)
        }))
    }
}

/// Get workout details
///
/// Get details of a specific workout
#[utoipa::path(
    get,
    path = "/workouts/{workout_id}",
    params(
        ("workout_id" = Uuid, Path, description = "Workout ID")
    ),
    responses(
        (status = 200, description = "Workout details retrieved successfully", body = WorkoutDetailsResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Workout not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "workouts",
    security(
        ("jwt_auth" = [])
    )
)]
#[get("/{workout_id}")]
pub async fn get_workout(
    workout_service: web::Data<WorkoutService>,
    user_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let user_id = user_id.into_inner();
    let workout_id = path.into_inner();
    
    match workout_service.get_workout(user_id, workout_id).await {
        Ok(workout) => HttpResponse::Ok().json(workout),
        Err(e) => {
            if e.to_string().contains("not found") {
                return HttpResponse::NotFound().json(serde_json::json!({
                    "error": "Workout not found"
                }));
            }
            
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get workout"
            }))
        }
    }
}

/// Get all workouts
///
/// Get all workouts for the authenticated user
#[utoipa::path(
    get,
    path = "/workouts",
    responses(
        (status = 200, description = "Workouts retrieved successfully", body = [Workout]),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    tag = "workouts",
    security(
        ("jwt_auth" = [])
    )
)]
#[get("")]
pub async fn get_workouts(
    workout_service: web::Data<WorkoutService>,
    user_id: web::ReqData<Uuid>,
) -> impl Responder {
    let user_id = user_id.into_inner();
    
    match workout_service.get_workouts(user_id).await {
        Ok(workouts) => HttpResponse::Ok().json(workouts),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to get workouts"
        }))
    }
}

/// Delete a workout
///
/// Delete a specific workout
#[utoipa::path(
    delete,
    path = "/workouts/{workout_id}",
    params(
        ("workout_id" = Uuid, Path, description = "Workout ID")
    ),
    responses(
        (status = 204, description = "Workout deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Workout not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "workouts",
    security(
        ("jwt_auth" = [])
    )
)]
#[delete("/{workout_id}")]
pub async fn delete_workout(
    workout_service: web::Data<WorkoutService>,
    user_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let user_id = user_id.into_inner();
    let workout_id = path.into_inner();
    
    match workout_service.delete_workout(user_id, workout_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            if e.to_string().contains("not found") {
                return HttpResponse::NotFound().json(serde_json::json!({
                    "error": "Workout not found"
                }));
            }
            
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete workout"
            }))
        }
    }
}

// Define a type for create workout response for Swagger documentation
#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct CreateWorkoutResponse {
    id: Uuid,
}
