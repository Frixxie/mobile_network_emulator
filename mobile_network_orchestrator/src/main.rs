use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    time::{Duration, SystemTime, UNIX_EPOCH},
    unimplemented,
};

use futures::StreamExt;
use geo::{EuclideanDistance, Point};
use mobile_network_core_event::{MobileNetworkCoreEvent, PdnConnectionStatus};
use mongodb::{bson::doc, Collection};
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
pub struct Ran {
    id: u32,
    radius: f64,
    x: f64,
    y: f64,
}

#[derive(Deserialize)]
pub struct EdgeDataCenter {
    id: u32,
    name: String,
    x: f64,
    y: f64,
}

impl Display for EdgeDataCenter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{}, {}, {},{}",
            self.id, self.name, self.x, self.y
        ))
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Application {
    id: u32,
    accesses: HashMap<String, Vec<Duration>>,
}

impl std::ops::Sub for Application {
    type Output = Application;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut hash_map = HashMap::new();
        for (ip, durations) in self.accesses {
            let rhs_durations = {
                match rhs.accesses.get(&ip) {
                    Some(durations) => durations.clone(),
                    None => Vec::new(),
                }
            };
            let result = durations
                .into_par_iter()
                .filter(|duration| {
                    rhs_durations
                        .iter()
                        .filter(|rhs_duration| *rhs_duration == duration)
                        .count()
                        == 0
                })
                .collect();

            hash_map.insert(ip.clone(), result);
        }
        Application {
            id: self.id,
            accesses: hash_map,
        }
    }
}

impl Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.id))
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

async fn fetch_rans(client: Client, url: &str) -> Result<Vec<Ran>, OrchestratorError> {
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
) -> Vec<(usize, Application)> {
    let mut applications: Vec<(usize, Application)> = Vec::with_capacity(10);
    for (i, edc) in edcs.iter().enumerate() {
        let mut new_applications =
            fetch_applications(client.clone(), &format!("{}/{}/applications", url, edc.id))
                .await
                .unwrap()
                .into_iter()
                .map(|application| (i, application))
                .collect();
        applications.append(&mut new_applications);
    }
    applications
}

async fn remove_application(application_id: u32, edc_id: usize, base_url: &str, client: Client) {
    match client
        .delete(format!(
            "{}/edge_data_centers/{}/applications/{}",
            base_url, edc_id, application_id
        ))
        .send()
        .await
    {
        Ok(result) => println!(
            "application with id {} deleting to edc {} with response: {}",
            application_id,
            edc_id,
            result.text().await.unwrap()
        ),
        Err(e) => println!("got error deleted application: {}, {}", application_id, e),
    }
}

async fn add_application(application_id: u32, edc_id: usize, base_url: &str, client: Client) {
    match client
        .post(format!(
            "{}/edge_data_centers/{}/applications/{}",
            base_url, edc_id, application_id
        ))
        .send()
        .await
    {
        Ok(result) => println!(
            "application with id {} added to edc {} with response: {}",
            application_id,
            edc_id,
            result.text().await.unwrap()
        ),
        Err(e) => println!("got error adding application: {}, {}", application_id, e),
    }
}

fn find_user_id(
    ip_addr: &str,
    timestamp_last_connected: &Duration,
    events: &[MobileNetworkCoreEvent],
) -> Option<(u32, Duration)> {
    events
        .into_par_iter()
        .filter_map(|event| match event.get_event() {
            mobile_network_core_event::Event::PdnConnectionEvent(pdn_connection_event) => {
                if pdn_connection_event.ipv4_addr.to_string() == ip_addr
                    && pdn_connection_event.status == PdnConnectionStatus::Created
                {
                    Some((event.get_user_id(), event.get_timestamp()))
                } else {
                    None
                }
            }
            mobile_network_core_event::Event::LocationReporting(_) => None,
        })
        .filter(|(_id, timestamp)| timestamp < timestamp_last_connected)
        .min_by(|(_id_a, timestamp_a), (_id_b, timestamp_b)| {
            (*timestamp_last_connected - *timestamp_a)
                .cmp(&(*timestamp_last_connected - *timestamp_b))
        })
}

fn find_ran(
    ip_addr: &str,
    timestamp_last_connected: &Duration,
    events: &[MobileNetworkCoreEvent],
) -> Option<Vec<(String, u32)>> {
    let id = match find_user_id(ip_addr, timestamp_last_connected, events) {
        Some(id) => id,
        None => return None,
    };
    let mut res = Vec::new();
    dbg!(id, *timestamp_last_connected - id.1);
    let position = events
        .into_par_iter()
        .filter_map(|event| match event.get_event() {
            mobile_network_core_event::Event::PdnConnectionEvent(_) => None,
            mobile_network_core_event::Event::LocationReporting(location_event) => {
                if event.get_user_id() == id.0 {
                    Some((
                        location_event.e_node_b_id.clone(),
                        event.get_timestamp(),
                        event.get_user_id(),
                    ))
                } else {
                    None
                }
            }
        })
        .max_by(|(_, t1, _), (_, t2, _)| t1.cmp(t2))
        .unwrap();
    res.push(position);
    Some(
        res.iter()
            .map(|(pos, _timestamp, id)| (pos.clone(), *id))
            .collect(),
    )
}

fn find_location(
    ip_addr: &str,
    timestamp_last_connected: &Duration,
    events: &[MobileNetworkCoreEvent],
) -> Option<Vec<(Point, u32)>> {
    let id = match find_user_id(ip_addr, timestamp_last_connected, events) {
        Some(id) => id,
        None => return None,
    };
    let mut res = Vec::new();
    dbg!(id, *timestamp_last_connected - id.1);
    let position = events
        .into_par_iter()
        .filter_map(|event| match event.get_event() {
            mobile_network_core_event::Event::PdnConnectionEvent(_) => None,
            mobile_network_core_event::Event::LocationReporting(location_event) => {
                if event.get_user_id() == id.0 {
                    match location_event.geographic_area {
                        mobile_network_core_event::GeographicArea::Point(p) => {
                            Some((p, event.get_timestamp(), event.get_user_id()))
                        }
                        _ => unimplemented!(),
                    }
                } else {
                    None
                }
            }
        })
        .max_by(|(_, t1, _), (_, t2, _)| t1.cmp(t2))
        .unwrap();
    res.push(position);
    Some(
        res.iter()
            .map(|(pos, _timestamp, id)| (*pos, *id))
            .collect(),
    )
}

async fn fetch_events(
    collection: &Collection<MobileNetworkCoreEvent>,
    _time: u64,
) -> Vec<MobileNetworkCoreEvent> {
    collection
        .find(doc! {})
        .await
        .unwrap()
        .collect::<Vec<Result<_, _>>>()
        .await
        .iter()
        .filter_map(|r| r.clone().ok())
        .collect()
}

fn find_edc(average_point: &Point, edcs: &[EdgeDataCenter]) -> Option<usize> {
    let mut min_index = 0;
    let mut min_length = f64::MAX;
    for (i, edc) in edcs.iter().enumerate() {
        let dist = Point::new(edc.x, edc.y)
            .euclidean_distance(average_point)
            .abs();
        if dist < min_length {
            min_index = i;
            min_length = dist;
            dbg!(min_length, min_index);
        }
    }
    if min_length < f64::MAX {
        Some(min_index)
    } else {
        None
    }
}

fn calculate_suggested_edc_weighted_avg(
    points: &[(Point, Vec<Duration>)],
    edcs: &[EdgeDataCenter],
) -> Option<usize> {
    let avg = match points
        .iter()
        .cloned()
        .reduce(|acc, (point, value)| (acc.0 + point * value.len() as f64, value))
    {
        Some(p) => {
            p.0 / points
                .iter()
                .map(|(_point, value)| value.len())
                .sum::<usize>() as f64
        }
        None => return None,
    };
    find_edc(&avg, edcs)
}

fn calculate_suggested_position_avg(
    points: &[(Point, Vec<Duration>)],
    edcs: &[EdgeDataCenter],
) -> Option<usize> {
    let avg = match points
        .iter()
        .cloned()
        .reduce(|acc, (point, value)| (acc.0 + point, value))
    {
        Some(p) => p.0 / points.len() as f64,
        None => return None,
    };
    find_edc(&avg, edcs)
}

fn distance_cost(ran: &Ran, edc: &EdgeDataCenter) -> f64 {
    Point::new(ran.x, ran.y)
        .euclidean_distance(&Point::new(edc.x, edc.y))
        .abs()
}

fn min_edc<'a, 'b, F>(
    application_usage: &'a [(Ran, usize)],
    edcs: &'b [EdgeDataCenter],
    cost_function: F,
) -> Option<&'b EdgeDataCenter>
where
    F: Fn(&Ran, &EdgeDataCenter) -> f64,
{
    edcs.iter()
        .map(|edc| {
            let cost: f64 = application_usage
                .iter()
                .map(|(ran, accesses)| *accesses as f64 * cost_function(ran, edc))
                .sum();
            (edc, cost)
        })
        //this should be ok as we do not expect NaNs
        .min_by(|(_edc, cost), (_rhs_edc, rhs_cost)| cost.partial_cmp(rhs_cost).unwrap())
        .map(|(edc, _cost)| edc)
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

    let mut applications: Vec<(usize, Application)> = fetch_all_applications(
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
            - 300;

        let events = fetch_events(&collection, time).await;

        let new_applications: Vec<(usize, Application)> = fetch_all_applications(
            client.clone(),
            &format!("{}{}", base_url, edge_data_center_url),
            &edge_data_centers,
        )
        .await;

        for (i, current_application) in applications.iter() {
            let (j, new_application) = new_applications
                .iter()
                .find(|(_edc_id, application)| application.id == current_application.id)
                .unwrap();

            dbg!(i, j, current_application.id, new_application.id);

            let diff = new_application.clone() - current_application.clone();

            let mut user_positions = Vec::new();
            for (ip, value) in diff.accesses.iter() {
                if !value.is_empty() {
                    dbg!(value.iter().max().unwrap(), events.len());
                    let points = match find_location(ip, value.iter().max().unwrap(), &events) {
                        Some(points) => points,
                        None => {
                            println!("failed to find id for ip {}", ip);
                            continue;
                        }
                    };
                    for (point, id) in points {
                        println!(
                            "{} with {}, should have pos ({},{})",
                            ip,
                            id,
                            point.x(),
                            point.y()
                        );
                        user_positions.push((point, value.clone()));
                    }
                }
            }
            if !user_positions.is_empty() {
                let edc_index =
                    calculate_suggested_edc_weighted_avg(&user_positions, &edge_data_centers)
                        .unwrap();
                if *j != edc_index {
                    println!("Moving application from {} to {}", j, edc_index);
                    remove_application(diff.id, *j, base_url, client.clone()).await;
                    add_application(diff.id, edc_index, base_url, client.clone()).await
                }
            }
        }

        applications = new_applications;
        tokio::time::sleep(Duration::from_secs(5)).await
    }
}
