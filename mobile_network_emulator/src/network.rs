use std::{error::Error, fmt::Display, time::{Duration, SystemTime}};

use geo::{EuclideanDistance, Point};
use serde::Serialize;

use crate::{application::Application, edge_data_center::EdgeDataCenter, pdu_session::PDUSession};

#[derive(Debug)]
pub struct NetworkError {
    message: String,
}

impl NetworkError {
    fn new(message: &str) -> Self {
        NetworkError {
            message: message.to_string(),
        }
    }
}

impl Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Error: {}", self.message))
    }
}

impl Error for NetworkError {}

#[derive(Debug, Clone, Serialize)]
pub struct NetworkLogEntry {
    user_id: u32,
    ip_address: String,
    time_used: u64,
    application_id: u32,
    timestamp: u64,
}

impl NetworkLogEntry {
    pub fn new(user_id: u32, ip_address: String, time_used: u64, application_id: u32) -> Self {
        Self {
            user_id,
            ip_address,
            time_used,
            application_id,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

pub struct Network {
    edge_data_centers: Vec<EdgeDataCenter>,
}

impl Network {
    pub fn new(edge_data_centers: Vec<EdgeDataCenter>) -> Self {
        Network { edge_data_centers }
    }

    pub fn use_application(
        &mut self,
        user: &PDUSession,
        application: &Application,
        ran_position: &Point,
    ) -> Result<NetworkLogEntry, NetworkError> {
        match self
            .edge_data_centers
            .iter_mut()
            .find(|edge_data_center| edge_data_center.contains_application(&application.id()))
        {
            Some(edge_data_center) => {
                //We know that the edge data center has the application.
                let delay = Self::generate_delay(ran_position, edge_data_center.get_position());
                let now = SystemTime::now();
                let _usage = edge_data_center
                    .use_application(*user.ip(), application)
                    .unwrap();

                let final_delay = now.elapsed().unwrap() + delay;
                let network_log_entry = NetworkLogEntry::new(
                    user.user().get_id(),
                    user.ip().to_string(),
                    final_delay.as_secs(),
                    application.id(),
                );

                Ok(network_log_entry)
            }
            None => Err(NetworkError::new(&format!(
                "Application with id {} does not exist",
                application.id()
            ))),
        }
    }

    pub fn get_edge_data_centers(&self) -> Vec<&EdgeDataCenter> {
        self.edge_data_centers.iter().collect()
    }

    pub fn get_edge_data_center(&self, id: u32) -> Option<&EdgeDataCenter> {
        match self
            .edge_data_centers
            .iter()
            .find(|edge_data_center| edge_data_center.get_id() == id)
        {
            Some(edge_data_center) => Some(edge_data_center),
            None => None,
        }
    }

    pub fn get_mut_edge_data_center(&mut self, id: u32) -> Option<&mut EdgeDataCenter> {
        match self
            .edge_data_centers
            .iter_mut()
            .find(|edge_data_center| edge_data_center.get_id() == id)
        {
            Some(edge_data_center) => Some(edge_data_center),
            None => None,
        }
    }

    pub fn get_applictions(&self) -> Vec<&Application> {
        self.edge_data_centers
            .iter()
            .flat_map(|edc| edc.get_applications())
            .collect()
    }

    pub fn get_total_application_usage(
        &self,
        edc_id: u32,
        application_id: u32,
    ) -> Result<u32, NetworkError> {
        let edc = match self
            .edge_data_centers
            .iter()
            .find(|edc| edc.get_id() == edc_id)
        {
            Some(edc) => edc,
            None => {
                return Err(NetworkError::new(&format!(
                    "Edge data center with id {} does not exist",
                    edc_id
                )))
            }
        };
        edc.get_total_uses_of_application(application_id)
            .map_err(|_| {
                NetworkError::new(&format!(
                    "Application with id {} does not exist",
                    application_id
                ))
            })
    }

    fn generate_delay(first_point: &Point, second_point: &Point) -> Duration {
        let distance = first_point.euclidean_distance(second_point).abs();
        Duration::new((distance * 1.5) as u64, 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ran::Ran, user::User};

    use super::*;
    use std::{iter::repeat, net::Ipv4Addr};

    use geo::Point;

    #[test]
    fn create() {
        let edge_data_centers = repeat((0, "Fredrik's edge data center", Point::new(0.0, 0.0)))
            .take(32)
            .map(|(id, name, position)| (EdgeDataCenter::new(id, name, position)))
            .collect();

        let network = Network::new(edge_data_centers);

        assert_eq!(network.edge_data_centers.len(), 32);
    }

    #[tokio::test]
    async fn use_application() {
        let mut edge_data_centers: Vec<EdgeDataCenter> =
            repeat((0, "Fredrik's edge data center", Point::new(0.0, 0.0)))
                .take(2)
                .map(|(id, name, position)| (EdgeDataCenter::new(id, name, position)))
                .collect();
        let application = Application::new(0);
        let ran = Ran::new(0, Point::new(1.0, 1.0), 50.0);
        let user = User::new(0, Point::new(1.0, 1.0), 1.0, &(-1.0..1.0));
        let pdu_session = PDUSession::new(
            user,
            std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            &ran,
        );
        edge_data_centers[0].add_application(0).unwrap();
        let mut network = Network::new(edge_data_centers);

        let result = network.use_application(&pdu_session, &application, &Point::new(1.0, 1.0));

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn use_application_not_present_should_fail() {
        let edge_data_centers: Vec<EdgeDataCenter> =
            repeat((0, "Fredrik's edge data center", Point::new(0.0, 0.0)))
                .take(1)
                .map(|(id, name, position)| (EdgeDataCenter::new(id, name, position)))
                .collect();
        let application = Application::new(0);

        let mut network = Network::new(edge_data_centers);
        let ran = Ran::new(0, Point::new(1.0, 1.0), 50.0);
        let user = User::new(0, Point::new(1.0, 1.0), 1.0, &(-1.0..1.0));
        let pdu_session = PDUSession::new(
            user,
            std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            &ran,
        );

        let result = network.use_application(&pdu_session, &application, &Point::new(1.0, 1.0));

        assert!(result.is_err());
    }
}
