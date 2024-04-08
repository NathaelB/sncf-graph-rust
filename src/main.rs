mod services;
mod controllers;
mod models;
mod pagination;

use tracing_actix_web::TracingLogger;

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use futures::TryStreamExt;
use mongodb::{Client, Collection};
use mongodb::bson::doc;
use mongodb::options::{AggregateOptions, ClientOptions};
use crate::models::stop::Stop;
use crate::controllers::stops_controller::{get_stop, test};

async fn get_mongo_collection() -> Collection<Stop> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();

    let client = Client::with_options(client_options).unwrap();
    let database = client.database("lirmm");
    database.collection::<Stop>("stops")
}

async fn get_mongo_client() -> Client {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    Client::with_options(client_options).unwrap()
}

async fn aggregate_location_type(collection: web::Data<Collection<Stop>>) -> impl Responder {
    let pipeline = vec![
        doc! {
            "$group": {
                "_id": "$location_type",
                "count": { "$sum": 1 },
            }
        }
    ];

    let options = AggregateOptions::builder().build();
    let mut cursor = collection.aggregate(pipeline, options).await.unwrap();

    let mut results = Vec::new();

    while let Some(result) = cursor.try_next().await.unwrap() {
        results.push(result);
    }

    HttpResponse::Ok().json(results)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let collection = get_mongo_collection().await;
    let client = get_mongo_client().await;
    let database = client.database("lirmm");





    let stop_service = web::Data::new(services::stop_service::StopService { collection: web::Data::new(database.collection("stops").clone()) });
    let stop_time_service = web::Data::new(services::stop_time_service::StopTimeService { collection: web::Data::new(database.collection("stops_time").clone()) });

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(collection.clone()))
            .app_data(stop_service.clone())
            .app_data(stop_time_service.clone())
            .service(controllers::health_controller::readiness)
            .service(get_stop)
            .service(test)
            .route("/stops", web::get().to(controllers::stops_controller::index))
            .route("/stops/location", web::get().to(aggregate_location_type))

    })
        .bind(("127.0.0.1", 3001))?
        .run()
        .await
}