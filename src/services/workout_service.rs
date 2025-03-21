use crate::db::DbPool;
use crate::models::{
    Workout, Exercise, WorkoutExercise,
    CreateWorkoutRequest, WorkoutDetailsResponse, WorkoutExerciseDetails,
};
use anyhow::{Result, anyhow};
use chrono::Utc;
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

/// Service for handling workout-related operations
pub struct WorkoutService {
    db_pool: DbPool,
}

impl WorkoutService {
    /// Create a new WorkoutService instance
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }
    
    /// Create a new workout
    pub async fn create_workout(&self, user_id: Uuid, req: CreateWorkoutRequest) -> Result<Uuid> {
        // Start a transaction
        let mut tx = self.db_pool.begin().await?;
        
        // Create the workout
        let workout_id = Uuid::new_v4();
        let now = Utc::now();
        
        sqlx::query!(
            r#"
            INSERT INTO workouts (id, user_id, name, description, date, duration, calories_burned, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            workout_id,
            user_id,
            req.name,
            req.description,
            req.date,
            req.duration,
            req.calories_burned,
            now,
            now
        )
        .execute(&mut *tx)
        .await?;
        
        // Create workout exercises
        for exercise in req.exercises {
            let workout_exercise_id = Uuid::new_v4();
            
            sqlx::query!(
                r#"
                INSERT INTO workout_exercises (id, workout_id, exercise_id, sets, reps, weight, duration, distance, notes, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                "#,
                workout_exercise_id,
                workout_id,
                exercise.exercise_id,
                exercise.sets,
                exercise.reps,
                exercise.weight,
                exercise.duration,
                exercise.distance,
                exercise.notes,
                now,
                now
            )
            .execute(&mut *tx)
            .await?;
        }
        
        // Commit the transaction
        tx.commit().await?;
        
        Ok(workout_id)
    }
    
    /// Get workout details by ID
    pub async fn get_workout(&self, user_id: Uuid, workout_id: Uuid) -> Result<WorkoutDetailsResponse> {
        // Get the workout
        let workout = sqlx::query_as!(
            Workout,
            r#"
            SELECT id, user_id, name, description, date, duration, calories_burned, created_at, updated_at
            FROM workouts
            WHERE id = $1 AND user_id = $2
            "#,
            workout_id,
            user_id
        )
        .fetch_optional(&self.db_pool)
        .await?
        .ok_or_else(|| anyhow!("Workout not found"))?;
        
        // Get the workout exercises
        let mut exercises = Vec::new();
        
        let workout_exercises = sqlx::query_as!(
            WorkoutExercise,
            r#"
            SELECT we.id, we.workout_id, we.exercise_id, we.sets, we.reps, we.weight, we.duration, we.distance, we.notes, we.created_at, we.updated_at
            FROM workout_exercises we
            WHERE we.workout_id = $1
            "#,
            workout_id
        )
        .fetch_all(&self.db_pool)
        .await?;
        
        for we in workout_exercises {
            // Get the exercise details
            let exercise = sqlx::query_as!(
                Exercise,
                r#"
                SELECT id, name, description, category, created_at, updated_at
                FROM exercises
                WHERE id = $1
                "#,
                we.exercise_id
            )
            .fetch_one(&self.db_pool)
            .await?;
            
            exercises.push(WorkoutExerciseDetails {
                id: we.id,
                exercise,
                sets: we.sets,
                reps: we.reps,
                weight: we.weight,
                duration: we.duration,
                distance: we.distance,
                notes: we.notes,
            });
        }
        
        Ok(WorkoutDetailsResponse {
            id: workout.id,
            name: workout.name,
            description: workout.description,
            date: workout.date,
            duration: workout.duration,
            calories_burned: workout.calories_burned,
            exercises,
        })
    }
    
    /// Get all workouts for a user
    pub async fn get_workouts(&self, user_id: Uuid) -> Result<Vec<Workout>> {
        let workouts = sqlx::query_as!(
            Workout,
            r#"
            SELECT id, user_id, name, description, date, duration, calories_burned, created_at, updated_at
            FROM workouts
            WHERE user_id = $1
            ORDER BY date DESC
            "#,
            user_id
        )
        .fetch_all(&self.db_pool)
        .await?;
        
        Ok(workouts)
    }
    
    /// Delete a workout
    pub async fn delete_workout(&self, user_id: Uuid, workout_id: Uuid) -> Result<PgQueryResult> {
        // Check if the workout exists and belongs to the user
        let workout = sqlx::query!(
            r#"
            SELECT id
            FROM workouts
            WHERE id = $1 AND user_id = $2
            "#,
            workout_id,
            user_id
        )
        .fetch_optional(&self.db_pool)
        .await?
        .ok_or_else(|| anyhow!("Workout not found"))?;
        
        // Delete the workout (cascade will delete related records)
        let result = sqlx::query!(
            r#"
            DELETE FROM workouts
            WHERE id = $1
            "#,
            workout_id
        )
        .execute(&self.db_pool)
        .await?;
        
        Ok(result)
    }
}
