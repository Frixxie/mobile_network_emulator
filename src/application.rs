use std::collections::HashMap;
use std::net::IpAddr;

use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Application {
    id: u32,
    times_used: HashMap<IpAddr, u32>,
}

impl Application {
    pub fn new(id: u32) -> Self {
        Application {
            id,
            times_used: HashMap::new(),
        }
    }

    pub fn id(&self) -> &u32 {
        &self.id
    }

    pub fn add_use(&mut self, ip_addr: IpAddr) {
        match self.times_used.get(&ip_addr) {
            Some(value) => {
                info!(
                    "Updated uses of application {} by ip {} with uses {}",
                    self.id,
                    ip_addr,
                    value + 1
                );
                self.times_used.insert(ip_addr, value + 1);
            }
            None => {
                info!(
                    "Updated uses of application {} by ip {} with uses {}",
                    self.id, ip_addr, 1
                );
                self.times_used.insert(ip_addr, 1);
            }
        };
    }

    pub fn get_use(&self, ip_addr: &IpAddr) -> u32 {
        match self.times_used.get(ip_addr) {
            Some(value) => *value,
            None => 0,
        }
    }

    pub fn get_total_usage(&self) -> u32 {
        self.times_used.values().sum()
    }

    pub fn dump_times_used(&self) -> Vec<(IpAddr, u32)> {
        self.times_used
            .iter()
            .map(|(ip, uses)| (*ip, *uses))
            .collect()
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

        assert_eq!(*id, 0);
    }

    #[test]
    fn add_get_use() {
        let mut application = Application::new(0);
        let ip_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        application.add_use(ip_addr);
        let use_count = application.get_use(&ip_addr);

        assert_eq!(use_count, 1);

        application.add_use(ip_addr);

        let use_count = application.get_use(&ip_addr);

        assert_eq!(use_count, 2);
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

    #[test]
    fn dump_times_used() {
        let mut application = Application::new(0);
        let ip_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        application.add_use(ip_addr);
        application.add_use(ip_addr);

        let times_used = application.dump_times_used();

        assert_eq!(times_used.len(), 1);
        assert_eq!(times_used[0].0, ip_addr);
        assert_eq!(times_used[0].1, 2);
    }
}
