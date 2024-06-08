use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use actix_web::{web, HttpResponse, Responder, HttpRequest, get};
use futures::TryStreamExt;
use juniper::GraphQLObject;
use mongodb::bson::doc;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::prelude::EdgeRef;
use serde::{Deserialize, Serialize};
use crate::models::stop::Stop;
use crate::models::stop_time::StopTime;
use crate::services::stop_service::StopService;
use crate::services::stop_time_service::StopTimeService;

#[derive(Serialize)]
struct NodeData {
    id: usize,
    name: String,
}

#[derive(Serialize)]
struct EdgeData {
    from: usize,
    to: usize,
    trips: HashSet<String>,
}




#[derive(Serialize)]
struct GraphData {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(get_stop);
    cfg.service(test);
}

#[get("/stops")]
pub async fn index(
    stop_service: web::Data<Arc<StopService>>,
    web::Query(info): web::Query<QueryParams>,
    req: HttpRequest,
) -> impl Responder {
    let base_url = format!("{}://{}{}", req.connection_info().scheme(), req.connection_info().host(), req.path());

    match stop_service.find_all(info.page.unwrap_or(1), info.size.unwrap_or(10), &base_url).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


#[get("/stops/{id}")]
pub async fn get_stop(
    stop_service: web::Data<Arc<StopService>>,
    id: web::Path<String>
) -> impl Responder {
    match stop_service.find_by_id(&id).await {
        Ok(Some(stop)) => HttpResponse::Ok().json(stop),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::NotFound().finish()
    }
}


#[get("/test")]
pub async fn test(
    stop_service: web::Data<Arc<StopService>>,
    stop_time_service: web::Data<StopTimeService>
) -> impl Responder {
    let lat_min = 42.5;
    let lat_max = 44.8;
    let lon_min = 1.5;
    let lon_max = 4.0;

    let cursor = stop_service.collection.find(
        Some(doc! {
            "stop_lat": { "$gte": lat_min, "$lte": lat_max },
            "stop_lon": { "$gte": lon_min, "$lte": lon_max }
        }),
        None
    ).await.unwrap();

    let data: Vec<Stop> = cursor.try_collect().await.unwrap();
    let stop_ids: Vec<String> = data.iter().map(|stop| stop.stop_id.clone()).collect();
    let stop_times = stop_time_service.find_by_stop_ids(&stop_ids).await.unwrap();

    /*let mut graph: DiGraph<String, HashSet<String>> = DiGraph::new();
    let mut stop_ids_to_node_index: HashMap<String, NodeIndex> = HashMap::new();

    for stop in data.iter() {
        let node_index = graph.add_node(stop.stop_name.clone());
        stop_ids_to_node_index.insert(stop.stop_id.clone(), node_index);
    }

    let mut stop_times_li = Vec::new();

    println!("{:?}", stop_ids_to_node_index.len());

    for stop_time in stop_times.iter() {
        if stop_ids_to_node_index.contains_key(&stop_time.stop_id) {
            stop_times_li.push(stop_time.clone());
        }
    }

    println!("StopTimesLi: {:?}", stop_times_li.len());

    let mut trip_to_stops: HashMap<String, Vec<&StopTime>> = HashMap::new();

    for stop_time in &stop_times_li {
        trip_to_stops.entry(stop_time.trip_id.clone())
            .or_insert(Vec::new())
            .push(stop_time);
    }



    for (trip_id, stops) in trip_to_stops {
        for window in stops.windows(2) {
            if let [from_stop, to_stop] = window {
                let from_node = stop_ids_to_node_index[&from_stop.stop_id];
                let to_node = stop_ids_to_node_index[&to_stop.stop_id];

                // i like create vec of all trip id for each edge between two nodes
                // example i have two nodes (A,B)
                // and 5 trips, of which 3 trips have a stop at A and B
                // i like :
                // edge A -> B : [trip1, trip2, trip3]

                let edge_index = graph.find_edge(from_node, to_node);
                if edge_index.is_none() {
                    graph.add_edge(from_node, to_node, HashSet::new());
                } else {
                    let edge = graph.edge_weight_mut(edge_index.unwrap()).unwrap();
                    edge.insert(trip_id.clone());
                }
                // "from": 336,
                // "to": 333,


            }
        }
    }

    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    for node_index in graph.node_indices() {
        let node_name = graph[node_index].clone();
        nodes.push(NodeData {
            id: node_index.index(),
            name: node_name,
        });
    }

    for edge_reference in graph.edge_references() {
        let from = edge_reference.source();
        let to = edge_reference.target();
        let trips = edge_reference.weight().clone();
        edges.push(EdgeData { from: from.index(), to: to.index(), trips });
    }

    let graph_data = GraphData { nodes, edges };

    println!("{:?}", graph.node_count());

    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "graph": graph_data,
        "stop_count": data.len(),
        "stop_times_count": stop_times.len(),
    }))*/

    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "stop_count": data.len(),
        "stop_times_count": stop_times.len(),
    }))
}

#[derive(Deserialize)]
pub struct QueryParams {
    pub size: Option<i64>,
    pub page: Option<i64>
}