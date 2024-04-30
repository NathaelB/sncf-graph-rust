use serde::{Deserialize, Serialize};
use crate::models::stop_time::StopTime;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Trip {
    pub route_id: String,
    pub service_id: String,
    pub trip_id: String,
    pub trip_headsign: String,
    pub direction_id: String,
    pub block_id: String,
    pub shape_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TripDetails {
    route_id: String,
    trip_headsign: String,
    stop_sequence: i32,
    stop_name: String,
    arrival_time: String,
    departure_time: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RouteTripsCount {
    route_short_name: String,
    route_long_name: String,
    direction_id: String,
    number_of_trips: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TripTransit {
    pub route_id: String,
    pub trip_headsign: String,
    pub stops_time: Vec<StopTime>
}