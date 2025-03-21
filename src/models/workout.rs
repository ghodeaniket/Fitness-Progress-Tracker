use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;

/// Workout model that maps to the workouts table
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Workout {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub date: DateTime<Utc>,
    pub duration: Option<i32>, // in seconds
    pub calories_burned: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Exercise model that maps to the exercises table
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Exercise {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// WorkoutExercise model that maps to the workout_exercises table
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct WorkoutExercise {
    pub id: Uuid,
    pub workout_id: Uuid,
    pub exercise_id: Uuid,
    pub sets: Option<i32>,
    pub reps: Option<i32>,
    pub weight: Option<f64>, // in kg
    pub duration: Option<i32>, // in seconds
    pub distance: Option<f64>, // in km
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create workout request
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateWorkoutRequest {
    #[validate(length(min = 1, max = 100))]
    #[schema(example = "Morning Cardio")]
    pub name: String,
    
    #[schema(example = "30 minute cardio session")]
    pub description: Option<String>,
    
    pub date: DateTime<Utc>,
    
    #[schema(example = 1800)]
    pub duration: Option<i32>,
    
    #[schema(example = 350)]
    pub calories_burned: Option<i32>,
    
    pub exercises: Vec<WorkoutExerciseInput>,
}

/// Workout exercise input for creating a workout
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct WorkoutExerciseInput {
    pub exercise_id: Uuid,
    
    #[schema(example = 3)]
    pub sets: Option<i32>,
    
    #[schema(example = 12)]
    pub reps: Option<i32>,
    
    #[schema(example = 20.5)]
    pub weight: Option<f64>,
    
    #[schema(example = 600)]
    pub duration: Option<i32>,
    
    #[schema(example = 5.5)]
    pub distance: Option<f64>,
    
    #[schema(example = "Felt good, increased weight")]
    pub notes: Option<String>,
}

/// Workout details response
#[derive(Debug, Serialize, ToSchema)]
pub struct WorkoutDetailsResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub date: DateTime<Utc>,
    pub duration: Option<i32>,
    pub calories_burned: Option<i32>,
    pub exercises: Vec<WorkoutExerciseDetails>,
}

/// Workout exercise details for response
#[derive(Debug, Serialize, ToSchema)]
pub struct WorkoutExerciseDetails {
    pub id: Uuid,
    pub exercise: Exercise,
    pub sets: Option<i32>,
    pub reps: Option<i32>,
    pub weight: Option<f64>,
    pub duration: Option<i32>,
    pub distance: Option<f64>,
    pub notes: Option<String>,
}
