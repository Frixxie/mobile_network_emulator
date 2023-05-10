use std::{collections::HashMap, error::Error, fmt::Display, time::Duration};

use geo::MultiPoint;
use mobile_network_core_event::MobileNetworkCoreEvent;
use reqwest::Client;
use serde::Deserialize;

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

#[derive(Deserialize)]
pub struct Application {
    id: u32,
    times_used: HashMap<String, usize>,
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

fn find_user_id(ip_addr: &str, events: &[MobileNetworkCoreEvent]) -> u32 {
    todo!();
    let pdn_connection_events = events.iter().filter(|event| {
    });

}

fn find_location(ip_addr: &str, events: &[MobileNetworkCoreEvent]) -> MultiPoint {
    todo!();
}

// pub trait DecideLocation<'a> {
//     fn decide(edcs: &'a [EdgeDataCenter], accesses: &[(String, usize)]) -> &'a EdgeDataCenter;
// }

#[tokio::main]
async fn main() {
    let base_url = "http://localhost:8080/network";
    let edge_data_center_url = "/edge_data_centers";
    let client = Client::new();
    loop {
        let edge_data_centers = fetch_edge_data_centers(
            client.clone(),
            &format!("{}{}", base_url, edge_data_center_url),
        )
        .await
        .unwrap();
        for edc in edge_data_centers.iter() {
            let applications = fetch_applications(
                client.clone(),
                &format!(
                    "{}{}/{}/applications",
                    base_url, edge_data_center_url, edc.id
                ),
            )
            .await
            .unwrap();
            println!("{}", edc);
            for application in applications {
                println!("Application with id: {}", application);
            }
        }

        tokio::time::sleep(Duration::from_secs(1)).await
    }
}
