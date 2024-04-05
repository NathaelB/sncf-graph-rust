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

async fn get_mongo_collection() -> Collection<Stop> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();

    let client = Client::with_options(client_options).unwrap();
    let database = client.database("lirmm");
    database.collection::<Stop>("stops")
}

/*async fn list_stops(
    collection: web::Data<Collection<Stop>>,
    params: web::Query<Pagination>,
    req: actix_web::HttpRequest
) -> impl Responder {
    let paginator = Paginator::new(params.page, params.limit);
    match paginator.paginate(&collection, &req).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}*/

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

    let stop_service = web::Data::new(services::stop_service::StopService { collection: web::Data::new(collection.clone()) });

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(collection.clone()))
            .app_data(stop_service.clone())
            .service(controllers::health_controller::readiness)
            .route("/stops", web::get().to(controllers::stops_controller::index))
            .route("/stops/location", web::get().to(aggregate_location_type))

    })
        .bind(("127.0.0.1", 3001))?
        .run()
        .await
}