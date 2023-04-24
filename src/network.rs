use std::{error::Error, fmt::Display, time::Duration};

use crate::{application::Application, edge_data_center::EdgeDataCenter};

#[derive(Debug)]
struct NetworkError {
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
    edge_data_centers: Vec<(EdgeDataCenter, Duration)>,
}

impl Network {
    pub fn new(edge_data_centers: Vec<(EdgeDataCenter, Duration)>) -> Self {
        Network { edge_data_centers }
    }

    pub fn use_application(&mut self, application: &Application) -> Result<(), NetworkError> {
        match self
            .edge_data_centers
            .iter_mut()
            .find(|(edge_data_center, _delay)| {
                edge_data_center.contains_application(application.url())
            }) {
            Some((edge_data_center, _duration)) => {
                //We know that the edge data center has the application.
                let _usage = edge_data_center.use_application(application).unwrap();
                Ok(())
            }
            None => Err(NetworkError::new(&format!(
                "Application on url {} does not exist",
                application.url()
            ))),
        }
    }

    pub fn get_edge_data_center(&self, id: usize) -> Option<&EdgeDataCenter> {
        match self
            .edge_data_centers
            .iter()
            .find(|(edge_data_center, _delay)| edge_data_center.get_id() == id)
        {
            Some((edge_data_center, _delay)) => Some(&edge_data_center),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::repeat;

    use geo::Point;
    use url::Url;

    #[test]
    fn create() {
        let edge_data_centers = repeat((
            0,
            "Fredrik's edge data center",
            Point::new(0.0, 0.0),
            Duration::from_secs(1),
        ))
        .take(32)
        .map(|(id, name, position, delay)| (EdgeDataCenter::new(id, name, position), delay))
        .collect();

        let network = Network::new(edge_data_centers);

        assert_eq!(network.edge_data_centers.len(), 32);
    }

    #[test]
    fn use_application() {
        let mut edge_data_centers: Vec<(EdgeDataCenter, Duration)> = repeat((
            0,
            "Fredrik's edge data center",
            Point::new(0.0, 0.0),
            Duration::from_secs(1),
        ))
        .take(2)
        .map(|(id, name, position, delay)| (EdgeDataCenter::new(id, name, position), delay))
        .collect();
        let application = Application::new(Url::parse("http://fasteraune.com").unwrap(), 0);
        edge_data_centers[0]
            .0
            .add_application(&application)
            .unwrap();
        let mut network = Network::new(edge_data_centers);

        let result = network.use_application(&application);

        assert!(result.is_ok());
    }

    #[test]
    fn use_application_not_present_should_fail() {
        let edge_data_centers: Vec<(EdgeDataCenter, Duration)> = repeat((
            0,
            "Fredrik's edge data center",
            Point::new(0.0, 0.0),
            Duration::from_secs(1),
        ))
        .take(1)
        .map(|(id, name, position, delay)| (EdgeDataCenter::new(id, name, position), delay))
        .collect();
        let application = Application::new(Url::parse("http://fasteraune.com").unwrap(), 0);

        let mut network = Network::new(edge_data_centers);

        let result = network.use_application(&application);

        assert!(result.is_err());
    }
}
