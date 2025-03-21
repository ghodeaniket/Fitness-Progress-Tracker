use crate::models::{
    UserRegisterRequest, UserLoginRequest, UserProfileResponse,
    CreateWorkoutRequest, WorkoutDetailsResponse, Workout
};
use utoipa::{
    OpenApi, 
    ToSchema, 
    openapi::security::{SecurityScheme, ApiKey, ApiKeyValue}
};
use utoipa::openapi::ComponentsBuilder;

/// Generate the OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Fitness Progress Tracker API",
        version = "1.0.0",
        description = "Backend API for tracking fitness progress"
    ),
    paths(
        crate::api::auth::register,
        crate::api::auth::login,
        crate::api::user::get_profile,
        crate::api::workout::create_workout,
        crate::api::workout::get_workout,
        crate::api::workout::get_workouts,
        crate::api::workout::delete_workout
    ),
    components(
        schemas(
            UserRegisterRequest,
            UserLoginRequest,
            UserProfileResponse,
            CreateWorkoutRequest, 
            WorkoutDetailsResponse,
            Workout
        ),
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "workouts", description = "Workout management endpoints")
    ),
    security(
        ("jwt_auth" = [])
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "jwt_auth", 
                SecurityScheme::ApiKey(ApiKey::Header(
                    ApiKeyValue::new("Authorization")
                ))
            );
        }
    }
}
