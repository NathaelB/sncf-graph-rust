use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Route {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub route_id: String,
    pub agency_id: Option<String>,
    pub route_short_name: String,
    pub route_long_name: String,
    pub route_desc: Option<String>,
    pub route_type: i32,
    pub route_url: Option<String>,
    pub route_color: Option<String>,
    pub route_text_color: Option<String>
}