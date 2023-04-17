use std::{error::Error, fmt::Display, time::Duration};

use url::Url;

use crate::edge_data_center::EdgeDataCenter;

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
        f.write_str(&format!("Error: {}", self.message).to_owned())
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

    pub fn use_application(&mut self, url: Url) -> Result<(), NetworkError> {
        todo!();
    }
}
