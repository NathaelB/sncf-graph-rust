use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Stop {
    stop_id: String,
    stop_name: String,
    stop_desc: Option<String>,
    stop_lat: f64,
    stop_lon: f64,
    zone_id: Option<String>,
    stop_url: Option<String>,
    location_type: String,
    parent_station: Option<String>
}