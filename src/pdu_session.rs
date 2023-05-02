use geo::Point;
use serde::{Serialize, ser::SerializeStruct};
use std::net::IpAddr;
use url::Url;

use crate::{network::Network, user::User};

#[derive(Debug, PartialEq, Clone)]
pub struct PDUSession {
    user: User,
    ip_address: IpAddr,
}

impl PDUSession {
    pub fn new(user: User, ip_address: IpAddr) -> Self {
        PDUSession { user, ip_address }
    }

    pub fn release(self) -> (User, IpAddr) {
        (self.user, self.ip_address)
    }

    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn update_user_position(&mut self) -> Point {
        self.user.next_pos()
    }

    pub fn ip(&self) -> &IpAddr {
        &self.ip_address
    }

    pub fn use_application(&self, _url: Url, _network: Network) {
        todo!();
    }
}

impl Serialize for PDUSession {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("PDUSession", 2)?;
        state.serialize_field("user", &self.user())?;
        state.serialize_field("ip", &self.ip())?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use super::*;

    #[test]
    fn create_release() {
        let user = User::new(1, Point::new(50.0, 50.0), 1.5, &(-50.0..50.0));
        let ip_address = Ipv4Addr::LOCALHOST;

        let pbu_session = PDUSession::new(user, std::net::IpAddr::V4(ip_address));
        let (user_1, ip_address_1) = pbu_session.release();
        assert_eq!(
            User::new(1, Point::new(50.0, 50.0), 1.5, &(-50.0..50.0)),
            user_1
        );
        assert_eq!(Ipv4Addr::LOCALHOST, ip_address_1);
    }
}
