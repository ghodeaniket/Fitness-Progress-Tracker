use crate::models::CreateWorkoutRequest;
use crate::services::WorkoutService;
use actix_web::{web, HttpResponse, Responder, get, post, delete};
use uuid::Uuid;
use validator::Validate;

/// Create a new workout
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

/// Get all workouts for a user
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
