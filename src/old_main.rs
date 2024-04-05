use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use csv::{Reader, ReaderBuilder};
use mongodb::bson::doc;
use mongodb::Client;
use mongodb::options::ClientOptions;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::prelude::EdgeRef;
use serde::{Deserialize, Serialize};


/**
On dispose d'une collection d'arrêt qui peuvent être utilisés pour des trajets,
un trajet est une entité qui relie une route (
 */
#[derive(Debug, Deserialize, Serialize)]
struct Stop {
    stop_id: String,
    stop_name: String,
    stop_desc: String,
    stop_lat: f64,
    stop_lon: f64,
    zone_id: String,
    stop_url: String,
    location_type: String,
    parent_station: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Trip {
    route_id: String,
    service_id: String,
    trip_id: String,
    trip_headsign: String,
    direction_id: String,
    block_id: String,
    shape_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct StopTime {
    trip_id: String,
    arrival_time: String,
    departure_time: String,
    stop_id: String,
    stop_sequence: usize,
}

#[derive(Debug, Deserialize, Serialize)]
struct Route {
    route_id: String,
}

fn find_stop_times<P: AsRef<Path>>(file_path: P) -> Result<Vec<StopTime>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    let mut stop_times = Vec::new();

    for result in rdr.deserialize() {
        let stop_time: StopTime = result?;

        if stop_time.trip_id.contains("OCESN876414F3178082:2024-03-22T00:43:46Z") {
            println!("{:?}", &stop_time);
            stop_times.push(stop_time);
        }
    }

    Ok(stop_times)
}


fn find_montpellier_stops<P: AsRef<Path>>(file_path: P) -> Result<Vec<Stop>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);
    let mut rdr = ReaderBuilder::new().from_reader(buf_reader);

    let mut montpellier_stops = Vec::new();

    for result in rdr.deserialize() {
        let stop: Stop = result?;

        if stop.stop_name.to_lowercase().contains("montpellier") {
            println!("{:?}", &stop);
            montpellier_stops.push(stop);
        }
    }

    Ok(montpellier_stops)
}

fn stops_in_herault<P: AsRef<Path>>(file_path: P) -> Result<Vec<Stop>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);

    let mut rdr = ReaderBuilder::new().from_reader(buf_reader);

    let mut stops_in_herault = Vec::new();

    // Bounding box for Hérault department, adjust as necessary
    let lat_min = 43.317;
    let lat_max = 43.982;
    let lon_min = 2.811;
    let lon_max = 4.295;

    for result in rdr.deserialize() {
        let stop: Stop = result?;
        if stop.stop_lat >= lat_min && stop.stop_lat <= lat_max && stop.stop_lon >= lon_min && stop.stop_lon <= lon_max {
            stops_in_herault.push(stop);
        }
    }

    Ok(stops_in_herault)
}


#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
/*    let mut graph: DiGraph<String, HashSet<String>> = DiGraph::new();
    let mut stop_ids_to_node_index: HashMap<String, NodeIndex> = HashMap::new();

    let lat_min = 42.5;
    let lat_max = 44.8;
    let lon_min = 1.5;
    let lon_max = 4.0;*/

    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    let database = client.database("lirmm");
    let collection = database.collection("trips");

    let mut rdr = ReaderBuilder::new().from_reader(BufReader::new(File::open("./data/trips.csv")?));

    for result in rdr.deserialize() {
        let trip: Trip = result.unwrap();

        let trip_document = doc! {
            "route_id": trip.route_id,
            "service_id": trip.service_id,
            "trip_id": trip.trip_id,
            "trip_headsign": trip.trip_headsign,
            "direction_id": trip.direction_id,
            "block_id": trip.block_id,
            "shape_id": trip.shape_id
        };
        collection.insert_one(trip_document, None).await?;

    }

    Ok(())

    //let mut rdr = ReaderBuilder::new().from_reader(BufReader::new(File::open("./data/stops.csv")?));
    /*for result in rdr.deserialize() {
        let stop: Stop = result.unwrap();

        let stop_document = doc! {
            "stop_id": stop.stop_id,
            "stop_name": stop.stop_name,
            "stop_desc": stop.stop_desc,
            "stop_lat": stop.stop_lat,
            "stop_lon": stop.stop_lon,
            "zone_id": stop.zone_id,
            "stop_url": stop.stop_url,
            "location_type": stop.location_type,
            "parent_station": stop.parent_station,
        };

        // Insertion du document dans la collection MongoDB
        collection.insert_one(stop_document, None).await?;





       /* if  stop.stop_lat >= lat_min && stop.stop_lat <= lat_max && stop.stop_lon >= lon_min && stop.stop_lon <= lon_max && !stop.parent_station.is_empty() && stop.stop_id.contains("OCETrain") {
            let node_index = graph.add_node(stop.stop_name.clone());
            stop_ids_to_node_index.insert(stop.stop_id, node_index);
        }*/
    }*/

    //let mut stop_times: Vec<StopTime> = Vec::new();
    /*let mut rdr = ReaderBuilder::new().from_reader(BufReader::new(File::open("./data/stop_times.csv")?));

    for result in rdr.deserialize() {
        let stop_time: StopTime = result.unwrap();

        let stop_document = doc! {
            "stop_id": stop_time.stop_id,
            "arrival_time": stop_time.arrival_time,
            "trip_id": stop_time.trip_id,
            "departure_time": stop_time.departure_time,
            "stop_sequence": stop_time.stop_sequence as i32,
        };

        // Insertion du document dans la collection MongoDB
        collection.insert_one(stop_document, None).await?;


        // if stop_ids_to_node_index.contains_key(&stop_time.stop_id) {
        //     stop_times.push(stop_time);
        // }
    }
    /*




    let mut trip_to_stops: HashMap<String, Vec<&StopTime>> = HashMap::new();

    for stop_time in &stop_times {
        trip_to_stops.entry(stop_time.trip_id.clone())
            .or_insert_with(Vec::new)
            .push(stop_time);
    }

    for (_trip_id, stops) in trip_to_stops {
        for window in stops.windows(2) {
            if let [from_stop, to_stop] = window {
                let from_node = stop_ids_to_node_index[&from_stop.stop_id];
                let to_node = stop_ids_to_node_index[&to_stop.stop_id];
                graph.add_edge(from_node, to_node, HashSet::new());
            }
        }
    }

    println!("Il existe plus de {:?} arrêt en Occitanie", graph.node_count());

    println!("Il y a {:?} arrête temporelles", stop_times.len());
*/

    /*for edge in graph.edge_references() {
        let source_node = edge.source();
        let target_node = edge.target();
        let edge_data = edge.weight();

        let source_stop_name = &graph[source_node];
        let target_stop_name = &graph[target_node];

        println!("Arête de '{}' à '{}'", source_stop_name, target_stop_name);
        for trip_id in edge_data {
            println!("   Trip ID: {}", trip_id);
        }
    }*/
    Ok(())

     */
}
