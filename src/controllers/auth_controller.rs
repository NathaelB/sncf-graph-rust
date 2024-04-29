use actix_web::{HttpResponse, post, Responder, web};
use serde::Deserialize;
use crate::services::authentication::AuthenticationService;

#[derive(Deserialize)]
pub struct AuthRequest {
    username: String,
    password: String,
}

#[post("/login")]
pub async fn login(
    auth_data: web::Json<AuthRequest>,
    auth_service: web::Data<AuthenticationService>
) -> Result<HttpResponse, actix_web::Error> {
    match auth_service.login(&auth_data.username, &auth_data.password).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(_) => Ok(HttpResponse::Unauthorized().finish())
    }
}