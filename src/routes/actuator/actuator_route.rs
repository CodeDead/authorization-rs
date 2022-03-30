use actix_web::{get, HttpResponse};

use crate::routes::actuator::dto::health_response::HealthResponse;

#[get("/health")]
pub async fn get_status() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse::new("UP"))
}
