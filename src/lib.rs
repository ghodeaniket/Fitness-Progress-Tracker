pub mod api;
pub mod config;
pub mod db;
pub mod models;
pub mod services;
pub mod utils;

use actix_web::{get, HttpResponse, Responder};

#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}
