use actix_web::{get, HttpResponse, Responder, web};
use crate::services::trip_service::TripService;

/**
 * This function is used to get the details of a trip
 *
 * @param trip_id: web::Path<String> - The trip id
 * @param trip_service: web::Data<TripService> - The trip service
 *
 * @return impl Responder - The response
 * @author - Nathael Bonnal
 */
#[get("/trips/{trip_id}/details")]
pub async fn get_trips_details(
    trip_id: web::Path<String>,
    trip_service: web::Data<TripService>,
) -> impl Responder {
    let trips = trip_service.find_trip_details(trip_id.as_str()).await;
    match trips {
        Ok(trips) => HttpResponse::Ok().json(trips),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}