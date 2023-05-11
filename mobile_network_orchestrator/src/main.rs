use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use futures::StreamExt;
use geo::Point;
use mobile_network_core_event::MobileNetworkCoreEvent;
use mongodb::{
    bson::{doc, serde_helpers::timestamp_as_u32},
    Collection,
};
use rayon::prelude::*;
use reqwest::Client;
use serde::Deserialize;
use simple_logger::SimpleLogger;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Orchestrator", about = "Orchestrator")]
struct Opt {
    #[structopt(short, long, default_value = "mongodb://localhost:27017/")]
    db_connection_string: String,
}

#[derive(Debug)]
pub struct OrchestratorError {
    message: String,
}

impl OrchestratorError {
    pub fn new(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
        }
    }
}

impl Display for OrchestratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for OrchestratorError {}

#[derive(Deserialize)]
pub struct EdgeDataCenter {
    id: u32,
    name: String,
    x: f64,
    y: f64,
}

impl Display for EdgeDataCenter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}, {}, {},{}", self.id, self.name, self.x, self.y).to_string())
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Application {
    id: u32,
    times_used: HashMap<String, usize>,
}

impl std::ops::Sub for Application {
    type Output = Application;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut hash_map = HashMap::new();
        for (key, value) in self.times_used {
            hash_map.insert(key.clone(), value - rhs.times_used.get(&key).unwrap_or(&0));
        }
        Application {
            id: self.id,
            times_used: hash_map,
        }
    }
}

impl Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.id).to_string())
    }
}

async fn fetch_edge_data_centers(
    client: Client,
    url: &str,
) -> Result<Vec<EdgeDataCenter>, OrchestratorError> {
    match client.get(url).send().await {
        Ok(response) => match response.json().await {
            Ok(res) => Ok(res),
            Err(e) => Err(OrchestratorError::new(&e.to_string())),
        },
        Err(e) => Err(OrchestratorError::new(&e.to_string())),
    }
}

async fn fetch_applications(
    client: Client,
    url: &str,
) -> Result<Vec<Application>, OrchestratorError> {
    match client.get(url).send().await {
        Ok(response) => match response.json().await {
            Ok(res) => Ok(res),
            Err(e) => Err(OrchestratorError::new(&e.to_string())),
        },
        Err(e) => Err(OrchestratorError::new(&e.to_string())),
    }
}

async fn fetch_all_applications(
    client: Client,
    url: &str,
    edcs: &[EdgeDataCenter],
) -> Vec<Application> {
    let mut applications: Vec<Application> = Vec::with_capacity(10);
    for edc in edcs.iter() {
        let mut new_applications =
            fetch_applications(client.clone(), &format!("{}/{}/applications", url, edc.id))
                .await
                .unwrap();
        applications.append(&mut new_applications);
    }
    applications
}

fn find_user_id(ip_addr: &str, events: &[MobileNetworkCoreEvent]) -> Vec<u32> {
    //TODO: timestamp may be needed here
    events
        .into_par_iter()
        .filter_map(|event| match event.get_event() {
            mobile_network_core_event::Event::PdnConnectionEvent(pdn_connection_event) => {
                if pdn_connection_event.ipv4_addr.to_string() == ip_addr {
                    Some(event.get_user_id())
                } else {
                    None
                }
            }
            mobile_network_core_event::Event::LocationReporting(_) => None,
        })
        .collect()
}

fn find_location(ip_addr: &str, events: &[MobileNetworkCoreEvent]) -> Vec<(Point, u32)> {
    let mut user_ids = find_user_id(ip_addr, events);
    user_ids.par_sort();
    user_ids.dedup();
    let mut res = Vec::new();
    for id in user_ids {
        let position = events
            .into_par_iter()
            .filter_map(|event| match event.get_event() {
                mobile_network_core_event::Event::PdnConnectionEvent(_) => None,
                mobile_network_core_event::Event::LocationReporting(location_event) => {
                    if event.get_user_id() == id.clone() {
                        match location_event.geographic_area {
                        mobile_network_core_event::GeographicArea::Point(p) => Some((p, event.get_timestamp(), event.get_user_id())),
                        mobile_network_core_event::GeographicArea::PointUncertainCircle => {
                            unimplemented!()
                        }
                        mobile_network_core_event::GeographicArea::PointUncertaintyEllipse => {
                            unimplemented!()
                        }
                        mobile_network_core_event::GeographicArea::Polygon(_) => unimplemented!(),
                        mobile_network_core_event::GeographicArea::PointAltitude => {
                            unimplemented!()
                        }
                        mobile_network_core_event::GeographicArea::PointAlititudeUncertainity => {
                            unimplemented!()
                        }
                        mobile_network_core_event::GeographicArea::EllipsoidArc => unimplemented!(),
                    }
                    } else {
                        None
                    }
                }
            }).max_by(|(_, timestamp_a, _), (_, timestamp_b, _)| timestamp_a.cmp(timestamp_b)).map(|(pos, _timestamp, id)| (pos, id)).unwrap();
        res.push(position);
    }
    res
}

#[tokio::main]
async fn main() {
    let base_url = "http://localhost:8080/network";
    let edge_data_center_url = "/edge_data_centers";
    let client = Client::new();

    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();
    let opts = Opt::from_args();
    let client_options = mongodb::options::ClientOptions::parse(&opts.db_connection_string)
        .await
        .unwrap();
    let db_client = mongodb::Client::with_options(client_options).unwrap();
    let database = db_client.database("mn_system");
    let collection: Collection<MobileNetworkCoreEvent> = database.collection("Events");

    let edge_data_centers = fetch_edge_data_centers(
        client.clone(),
        &format!("{}{}", base_url, edge_data_center_url),
    )
    .await
    .unwrap();

    let mut applications: Vec<Application> = fetch_all_applications(
        client.clone(),
        &format!("{}{}", base_url, edge_data_center_url),
        &edge_data_centers,
    )
    .await;

    loop {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - 60;
        let events: Vec<MobileNetworkCoreEvent> = collection
            .find(
                doc! {
                    "timestamp": doc! {
                        "$gt": time as u32,

                    }
                },
                None,
            )
            .await
            .unwrap()
            .collect::<Vec<Result<_, _>>>()
            .await
            .iter()
            .filter_map(|r| r.clone().ok())
            .collect();

        let new_applications: Vec<Application> = fetch_all_applications(
            client.clone(),
            &format!("{}{}", base_url, edge_data_center_url),
            &edge_data_centers,
        )
        .await;

        for (old, new) in applications.iter().zip(new_applications.iter()) {
            println!("{}, {}", old.id, new.id);
            let diff = new.clone() - old.clone();

            println!("Application with id: {}", diff);
            for (ip, value) in diff.times_used.iter() {
                if value > &0 {
                    let points = find_location(ip, &events);
                    for (point, id) in points {
                        println!("{} with {}, should have pos ({},{})", ip, id, point.x(), point.y());
                    }
                }
            }
        }

        applications = new_applications;
        tokio::time::sleep(Duration::from_secs(30)).await
    }
}
