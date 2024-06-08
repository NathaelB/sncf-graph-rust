mod services;
mod controllers;
mod models;
mod pagination;

use std::sync::Arc;
use env_logger::Env;
use actix_web::{App, HttpServer, middleware, Responder, web};
use actix_web_middleware_keycloak_auth::{KeycloakAuth, DecodingKey};
use futures::TryStreamExt;
use mongodb::{Client};
use mongodb::options::{ClientOptions};
use serde::Deserialize;
use crate::services::authentication::Config;
use crate::services::stop_service::StopService;

#[derive(Debug, Deserialize)]
pub struct WellKnowResponse {
    pub keys: Vec<Key>,
}

#[derive(Debug, Deserialize)]
pub struct Claims {
    pub sub: String,
}

#[derive(Debug, Deserialize)]
pub struct Key {
    pub kid: String,
    pub kty: String,
    pub alg: String,
    pub n: String,
    pub e: String,
    pub x5c: Vec<String>,
    pub x5t: String,
    #[serde(rename = "x5t#S256")]
    pub x5t_s256: String,
}

async fn get_mongo_client() -> Client {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    Client::with_options(client_options).unwrap()
}


async fn fetch_oidc_cert() -> Result<String, Box<dyn std::error::Error>> {
    let url = "http://localhost:8080/realms/sncf/protocol/openid-connect/certs";
    let client = reqwest::Client::new();

    let res = client.get(&*url).send().await?;
    let data: WellKnowResponse = res.json().await?;

    let rs256key = data.keys.into_iter().find(|key| key.alg == "RS256");

    match rs256key {
        Some(key) => Ok(key.x5c[0].clone()),
        None => Err("No RS256 key found".into())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = get_mongo_client().await;
    let database = client.database("lirmm");

    std::env::set_var("RUST_LOG", "info,actix_web_middleware_keycloak_auth=trace");
    env_logger::init();

    let mongo_client = get_mongo_client().await;

    let mongo_client = Arc::new(mongo_client);

    let stop_service = StopService::new(mongo_client.clone());


    let stop_time_service = web::Data::new(services::stop_time_service::StopTimeService { collection: web::Data::new(database.collection("stops_time").clone()) });
    let route_service = web::Data::new(services::route_service::RouteService { collection: web::Data::new(database.collection("routes").clone()) });
    let trip_service = web::Data::new(services::trip_service::TripService { collection: web::Data::new(database.collection("trips").clone()) });

    let auth_service = web::Data::new(services::authentication::AuthenticationService::new(Config {
        url: "http://localhost:8080".to_string(),
        realm: "sncf".to_string(),
        client_id: "api".to_string(),
        client_secret: "7rSqywlG1Rmi5EAMBUJxSRIhxvlcZBO3".to_string()
    }));

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(auth_service.clone())
            .service(controllers::health_controller::readiness)
            .service(controllers::auth_controller::login)
            .service(
                web::scope("")
                    //.wrap(keycloak_auth.clone())
                    .app_data(web::Data::new(stop_service.clone()))
                    .app_data(stop_time_service.clone())
                    .app_data(route_service.clone())
                    .app_data(trip_service.clone())
                    //.service(controllers::routes_controller::get_trips_count_by_route)
                    .configure(controllers::stops_controller::init_routes)
                    .configure(controllers::routes_controller::init_routes)
                    .service(controllers::trips_controller::get_trips_details)
                    .service(controllers::routes_controller::upload)
            )

    })
        .bind(("127.0.0.1", 3001))?
        .workers(1)
        .run()
        .await
}