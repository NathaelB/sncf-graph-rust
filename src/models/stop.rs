use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Stop {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub stop_id: String,
    pub stop_name: String,
    pub stop_desc: Option<String>,
    pub stop_lat: f64,
    pub stop_lon: f64,
    pub zone_id: Option<String>,
    pub stop_url: Option<String>,
    pub location_type: String,
    pub parent_station: Option<String>
}