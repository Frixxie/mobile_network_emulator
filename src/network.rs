use std::{error::Error, fmt::Display, time::Duration};

use geo::{EuclideanDistance, Point};

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

pub struct Network {
    edge_data_centers: Vec<EdgeDataCenter>,
}

impl Network {
    pub fn new(edge_data_centers: Vec<EdgeDataCenter>) -> Self {
        Network { edge_data_centers }
    }

    //TODO: Change this to take in position as well
    pub async fn use_application(
        &mut self,
        user: &PDUSession,
        application: &Application,
        _ran_position: &Point,
    ) -> Result<(), NetworkError> {
        match self
            .edge_data_centers
            .iter_mut()
            .find(|edge_data_center| edge_data_center.contains_application(application.id()))
        {
            Some(edge_data_center) => {
                //We know that the edge data center has the application.
                // let delay = Self::generate_delay(&ran_position, edge_data_center.get_position());
                // tokio::time::sleep(delay).await;
                let _usage = edge_data_center
                    .use_application(*user.ip(), application)
                    .unwrap();
                Ok(())
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
        _edc_id: u32,
        _application_id: u32,
    ) -> Result<u32, NetworkError> {
        Ok(1)
    }

    pub fn get_application_usage(
        &self,
        _edc_id: u32,
        _application_id: u32,
    ) -> Result<u32, NetworkError> {
        Ok(1)
    }

    fn generate_delay(first_point: &Point, second_point: &Point) -> Duration {
        let distance = first_point.euclidean_distance(second_point).abs();
        Duration::new((distance * 2.0) as u64, 0)
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

        let result = network
            .use_application(&pdu_session, &application, &Point::new(1.0, 1.0))
            .await;

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

        let result = network
            .use_application(&pdu_session, &application, &Point::new(1.0, 1.0))
            .await;

        assert!(result.is_err());
    }
}
