use actix_web::web;
use mongodb::bson::doc;
use mongodb::Collection;
use futures::TryStreamExt;
use futures_util::StreamExt;
use crate::models::trip::{RouteTripsCount, Trip, TripDetails, TripTransit};
use crate::pagination::{PaginationBuilder, PaginationResponse};

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

    pub async fn count_trips_by_route_and_direction(&self) -> mongodb::error::Result<Vec<RouteTripsCount>> {
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

    pub async fn count_trips(&self, route_id: &str) -> mongodb::error::Result<i32> {
        let pipeline = vec![
            doc! { "$match": { "route_id": route_id } },
            doc! { "$count": "total" }
        ];

        let mut cursor = self.collection.aggregate(pipeline, None).await?;
        if let Some(result) = cursor.next().await {
            let doc = result?;
            println!("{:?}", doc);
            let total = doc.get_i32("total").unwrap();
            Ok(total)
        } else {
            Ok(0)
        }
    }

    pub async fn get_trips_with_stops_sorted(&self, route_id: &str, total: u64, page: i64, size: i64, url: String) -> Result<PaginationResponse<TripTransit>, mongodb::error::Error> {
        let skip = (page - 1) * size;

        let pipeline = vec![
            doc! { "$match": { "route_id": route_id } },
            doc! { "$lookup": {
                "from": "stops_time",
                "localField": "trip_id",
                "foreignField": "trip_id",
                "as": "stops_time_list"
            }},
            doc! { "$unwind": "$stops_time_list" },
            doc! { "$sort": { "stops_time_list.stop_sequence": 1 } },
            doc! { "$group": {
                "_id": "$trip_id",
                "route_id": { "$first": "$route_id" },
                "trip_headsign": { "$first": "$trip_headsign" },
                "stops_time": { "$push": "$stops_time_list" }
            }},
            doc! { "$sort": { "_id": 1 } },
            doc! { "$limit": size },
            doc! { "$skip": skip }
        ];

        let mut cursor = self.collection.aggregate(
            pipeline, None
        ).await.unwrap();
        let mut results = Vec::new();

        while let Some(doc) = cursor.next().await {
            let trip_transit: TripTransit = mongodb::bson::from_bson(mongodb::bson::Bson::Document(doc.unwrap())).unwrap();
            results.push(trip_transit);
        }

        let builder = PaginationBuilder::new(results, total, page, size, url);
        Ok(builder.build_response())
    }
}