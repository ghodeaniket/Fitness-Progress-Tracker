pub mod auth;
pub mod middleware;
pub mod user;
pub mod workout;

use actix_web::web;

/// Configure API routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // Authentication routes
    cfg.service(
        web::scope("/auth")
            .service(auth::register)
            .service(auth::login)
    );
    
    // User routes
    cfg.service(
        web::scope("/users")
            .service(user::get_profile)
    );
    
    // Workout routes
    cfg.service(
        web::scope("/workouts")
            .service(workout::create_workout)
            .service(workout::get_workout)
            .service(workout::get_workouts)
            .service(workout::delete_workout)
    );
}
