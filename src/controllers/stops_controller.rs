use actix_web::{web, HttpResponse, Responder, HttpRequest};
use serde::{Deserialize};
use crate::services::stop_service::StopService;

pub async fn index(
    stop_service: web::Data<StopService>,
    web::Query(info): web::Query<QueryParams>,
    req: HttpRequest,
) -> impl Responder {
    let base_url = format!("{}://{}{}", req.connection_info().scheme(), req.connection_info().host(), req.path());

    match stop_service.find_all(info.page.unwrap_or(1), info.size.unwrap_or(10), &base_url).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Deserialize)]
pub struct QueryParams {
    pub size: Option<i64>,
    pub page: Option<i64>
}