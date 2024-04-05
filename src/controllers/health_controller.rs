use actix_web::{get, HttpResponse, Responder};

#[get("/health/ready")]
pub async fn readiness() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok"
    }))
}