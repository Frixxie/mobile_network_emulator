use geo::Point;
use serde::{ser::SerializeStruct, Serialize};
use std::{net::IpAddr, sync::Arc};

use crate::{ran::Ran, user::User};

#[derive(Debug, PartialEq, Clone)]
pub struct PDUSession {
    user: User,
    ip_address: IpAddr,
    ran: Arc<Ran>,
}

impl PDUSession {
    pub fn new(user: User, ip_address: IpAddr, ran: &Ran) -> Self {
        PDUSession {
            user,
            ip_address,
            ran: Arc::new(ran.clone()),
        }
    }

    pub fn release(self) -> (User, IpAddr) {
        (self.user, self.ip_address)
    }

    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn user_mut(&mut self) -> &mut User {
        &mut self.user
    }

    pub fn update_user_position(&mut self) -> Point {
        self.user.next_pos()
    }

    pub fn ip(&self) -> &IpAddr {
        &self.ip_address
    }

    pub fn get_ran(&self) -> Arc<Ran> {
        self.ran.clone()
    }
}

impl Serialize for PDUSession {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("PDUSession", 3)?;
        state.serialize_field("user", &self.user())?;
        state.serialize_field("ip", &self.ip())?;
        state.serialize_field("ran", &self.ran.get_id())?;
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
        let ran = Ran::new(0, Point::new(0.0, 0.0), 100.0);

        let pbu_session = PDUSession::new(user, std::net::IpAddr::V4(ip_address), &ran);
        let (user_1, ip_address_1) = pbu_session.release();
        assert_eq!(
            User::new(1, Point::new(50.0, 50.0), 1.5, &(-50.0..50.0)),
            user_1
        );
        assert_eq!(Ipv4Addr::LOCALHOST, ip_address_1);
    }
}
