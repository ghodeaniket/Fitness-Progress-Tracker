mod api;
mod config;
mod db;
mod models;
mod services;
mod utils;

use actix_web::{web, App, HttpServer, Responder, HttpResponse, get, middleware::Logger};
use log::{info, error};
use std::process::exit;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::middleware::JwtAuth;
use crate::config::AppConfig;
use crate::db::init_db;
use crate::services::{UserService, WorkoutService};
use crate::api::docs::ApiDoc;

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment variables from .env file
    dotenv::dotenv().ok();
    
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    // Load configuration
    let config = match AppConfig::from_env() {
        Ok(config) => {
            info!("Configuration loaded successfully");
            config
        },
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            exit(1);
        }
    };
    
    // Initialize database
    let db_pool = match init_db(&config.database_url).await {
        Ok(pool) => {
            info!("Database initialized successfully");
            pool
        },
        Err(e) => {
            error!("Failed to initialize database: {}", e);
            exit(1);
        }
    };
    
    // Create services
    let user_service = UserService::new(
        db_pool.clone(),
        config.jwt_secret.clone(),
        config.jwt_expiration,
    );
    
    let workout_service = WorkoutService::new(db_pool.clone());
    
    // Create JWT middleware
    let jwt_middleware = JwtAuth::new(config.jwt_secret.clone());
    
    // Create the OpenAPI document
    let openapi = ApiDoc::openapi();
    
    // Start HTTP server
    info!("Starting server at {}", config.server_addr());
    HttpServer::new(move || {
        App::new()
            // Enable logger middleware
            .wrap(Logger::default())
            // Register the shared database pool
            .app_data(web::Data::new(db_pool.clone()))
            // Register services
            .app_data(web::Data::new(user_service.clone()))
            .app_data(web::Data::new(workout_service.clone()))
            // Register the health check endpoint
            .service(health_check)
            // Serve Swagger UI
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone())
            )
            // Register public API routes
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/auth")
                            .service(api::auth::register)
                            .service(api::auth::login)
                    )
            )
            // Register protected API routes with JWT middleware
            .service(
                web::scope("/api/v1")
                    .wrap(jwt_middleware.clone())
                    .service(
                        web::scope("/users")
                            .service(api::user::get_profile)
                    )
                    .service(
                        web::scope("/workouts")
                            .service(api::workout::create_workout)
                            .service(api::workout::get_workout)
                            .service(api::workout::get_workouts)
                            .service(api::workout::delete_workout)
                    )
            )
    })
    .bind(config.server_addr())?
    .run()
    .await
}
