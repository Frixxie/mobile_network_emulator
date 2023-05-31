use std::net::IpAddr;
use std::time::{SystemTime, UNIX_EPOCH};
use std::vec;
use std::{collections::HashMap, time::Duration};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Application {
    id: u32,
    accesses: HashMap<IpAddr, Vec<Duration>>,
}

impl Application {
    pub fn new(id: u32) -> Self {
        Application {
            id,
            accesses: HashMap::new(),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn add_use(&mut self, ip_addr: IpAddr) {
        match self.accesses.get_mut(&ip_addr) {
            Some(durations) => {
                durations.push(SystemTime::now().duration_since(UNIX_EPOCH).unwrap());
            }
            None => {
                let durations = vec![SystemTime::now().duration_since(UNIX_EPOCH).unwrap()];
                self.accesses.insert(ip_addr, durations);
            }
        };
    }

    pub fn get_use(&self, ip_addr: &IpAddr) -> Vec<Duration> {
        match self.accesses.get(ip_addr) {
            Some(durations) => durations.clone(),
            None => Vec::new(),
        }
    }

    pub fn get_total_usage(&self) -> u32 {
        self.accesses
            .values()
            .map(|durations| durations.len())
            .sum::<usize>()
            .try_into()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use super::*;

    #[test]
    fn new() {
        let application = Application::new(0);
        let id = application.id();

        assert_eq!(id, 0);
    }

    #[test]
    fn add_get_use() {
        let mut application = Application::new(0);
        let ip_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        application.add_use(ip_addr);
        let use_count = application.get_use(&ip_addr);

        assert_eq!(use_count.len(), 1);

        application.add_use(ip_addr);

        let use_count = application.get_use(&ip_addr);

        assert_eq!(use_count.len(), 2);
    }

    #[test]
    fn get_total_usage() {
        let mut application = Application::new(0);
        let ip_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        application.add_use(ip_addr);
        application.add_use(ip_addr);

        let total_usage = application.get_total_usage();

        assert_eq!(total_usage, 2);
    }
}
