use std::fmt::format;
use std::io::Cursor;
use actix_multipart::Multipart;
use actix_web::{get, HttpResponse, post, Responder, web};
use futures::{AsyncWriteExt, StreamExt, TryStreamExt};
use crate::controllers::stops_controller::QueryParams;
use crate::models::route::Route;
use crate::services::route_service::RouteService;
use crate::services::trip_service::TripService;

/*#[get("/routes")]
pub async fn index(
    route_service: web::Data<RouteService>
) -> impl Responder {

}*/


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_trips_count_by_route);
    cfg.service(get_trips_details_by_route_id);
    cfg.service(upload);
}

#[get("/routes/trips")]
pub async fn get_trips_count_by_route(
    trip_service: web::Data<TripService>,
    info: web::Query<QueryParams>,
    route_service: web::Data<RouteService>
) -> impl Responder {
    let url = "/routes/trips".to_string();
    if let Ok(len) = route_service.count_routes().await {
        match trip_service.count_trips_by_route_and_direction(
            len as u64, info.page.unwrap_or(1),
            info.size.unwrap_or(10), url
        ).await {
            Ok(response) => HttpResponse::Ok().json(response),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    } else {
        HttpResponse::InternalServerError().finish()
    }

}

#[get("/routes/{route_id}/trips")]
pub async fn get_trips_details_by_route_id(
    trip_service: web::Data<TripService>,
    info: web::Query<QueryParams>,
    route_id: web::Path<String>,
) -> impl Responder {
    let url = format!("/routes/{}/trips", &route_id.as_str());
    if let Ok(len) = trip_service.count_trips(&route_id).await {
        match trip_service.get_trips_with_stops_sorted(
            route_id.as_str(),
            len as u64, info.page.unwrap_or(1),
            info.size.unwrap_or(10), url
        ).await {
            Ok(response) => HttpResponse::Ok().json(response),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    } else {
        HttpResponse::InternalServerError().finish()
    }

}

#[post("/routes/csv")]
pub async fn upload(
    route_service: web::Data<RouteService>,
    mut payload: Multipart
) -> Result<HttpResponse, actix_web::Error> {

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();
        let filename = content_disposition.get_filename().unwrap();

        let mut csv_data = Vec::new();

        while let Some(chunk) = field.try_next().await? {
            csv_data.extend_from_slice(&chunk);
        }

        let csv_string = String::from_utf8(csv_data).unwrap();
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(csv_string.as_bytes());


        match rdr.headers() {

            Ok(headers) => {
                let header_values: Vec<&str> = headers.iter().collect();
                let expected_headers = vec!["route_id", "agency_id", "route_short_name", "route_long_name", "route_desc", "route_type", "route_url", "route_color", "route_text_color"];


                if header_values != expected_headers {
                    return Ok(HttpResponse::BadRequest().body("Invalid headers"));
                }


                for result in rdr.deserialize::<Route>() {
                    match result {
                        Ok(route) => {
                            println!("[?] Enregistrement de la route {:?}", &route.route_id);
                            route_service.create(route).await.unwrap();

                        },
                        Err(_) => {
                            return Ok(HttpResponse::BadRequest().body("Invalid CSV file"));
                        }
                    }
                }

            },
            Err(_) => {
                return Ok(HttpResponse::BadRequest().body("Invalid CSV file"));
            }
        }
    }
    Ok(HttpResponse::Ok().body("File uploaded successfully!"))
}