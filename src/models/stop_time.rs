use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StopTime {
    pub trip_id: String,
    pub arrival_time: String,
    pub departure_time: String,
    pub stop_id: String,
    pub stop_sequence: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StopTimeDetail {
    pub arrival_time: String,
    pub departure_time: String,
    pub route_short_name: String,
    pub route_long_name: String,
    pub trip_headsign: String,
    pub direction_id: String,
}