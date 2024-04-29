use actix_web::web;
use mongodb::bson::doc;
use mongodb::Collection;
use futures::TryStreamExt;
use futures_util::StreamExt;
use crate::models::trip::{RouteTripsCount, Trip, TripDetails};

pub struct TripService {
    pub collection: web::Data<Collection<Trip>>
}

impl TripService {
    /**
    * This function is used to find all the trips
    *
    * @param &self - The reference to the trip service
    *
    * @return Result<Vec<Trip>, mongodb::error::Error> - The result of the operation
    */
    pub async fn find_trip_details(&self, trip_id: &str) -> mongodb::error::Result<Vec<TripDetails>> {
        let pipeline = vec![
            doc! { "$match": { "trip_id": trip_id } },
            doc! { "$lookup": { "from": "stops_time", "localField": "trip_id", "foreignField": "trip_id", "as": "stops_time" } },
            doc! { "$unwind": "$stops_time" },
            doc! { "$lookup": { "from": "stops", "localField": "stops_time.stop_id", "foreignField": "stop_id", "as": "stop_details" } },
            doc! { "$unwind": "$stop_details" },
            doc! { "$project": {
                "route_id": 1,
                "trip_headsign": 1,
                "stop_sequence": "$stops_time.stop_sequence",
                "stop_name": "$stop_details.stop_name",
                "arrival_time": "$stops_time.arrival_time",
                "departure_time": "$stops_time.departure_time"
            }},
            doc! { "$sort": { "stops_time.stop_sequence": 1 } }
        ];

        let mut cursor = self.collection.aggregate(pipeline, None).await?;
        let mut results = Vec::new();

        while let Some(doc) = cursor.next().await {
            let trip_details: TripDetails = mongodb::bson::from_bson(mongodb::bson::Bson::Document(doc?))?;
            results.push(trip_details);
        }
        Ok(results)
    }

    pub async fn cout_trips_by_route_and_direction(&self) -> mongodb::error::Result<Vec<RouteTripsCount>> {
        let pipeline = vec![
            doc! { "$group": { "_id": { "route_id": "$route_id", "direction_id": "$direction_id" }, "number_of_trips": { "$sum": 1 } } },
            doc! { "$lookup": { "from": "routes", "localField": "_id.route_id", "foreignField": "route_id", "as": "route_details" } },
            doc! { "$unwind": "$route_details" },
            doc! { "$project": {
                "route_short_name": "$route_details.route_short_name",
                "route_long_name": "$route_details.route_long_name",
                "direction_id": "$_id.direction_id",
                "number_of_trips": 1
            }},
            doc! { "$sort": { "number_of_trips": -1 } }
        ];

        let mut cursor = self.collection.aggregate(pipeline, None).await?;
        let mut results = Vec::new();

        while let Some(doc) = cursor.next().await {
            let route_trips_count: RouteTripsCount = mongodb::bson::from_bson(mongodb::bson::Bson::Document(doc?))?;
            results.push(route_trips_count);
        }

        Ok(results)
    }
}