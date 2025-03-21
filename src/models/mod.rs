// Export all model modules
pub mod user;
pub mod workout;

// Re-export common model types for convenience
pub use user::{User, UserRegisterRequest, UserLoginRequest, UserProfileResponse, Claims};
pub use workout::{
    Workout, Exercise, WorkoutExercise,
    CreateWorkoutRequest, WorkoutExerciseInput,
    WorkoutDetailsResponse, WorkoutExerciseDetails,
};
