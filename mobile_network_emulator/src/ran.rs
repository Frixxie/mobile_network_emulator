use geo::Contains;
use geo::EuclideanDistance;
use geo::Point;
use serde::ser::SerializeStruct;
use serde::Serialize;

use crate::{pdu_session::PDUSession, user::User};

#[derive(Debug, Clone, PartialEq)]
pub struct Ran {
    id: u32,
    position: Point,
    radius: f64,
    connected_users: Vec<PDUSession>,
}

impl Ran {
    pub fn new(id: u32, position: Point, radius: f64) -> Self {
        Ran {
            id,
            position,
            radius,
            connected_users: Vec::new(),
        }
    }

    fn get_connected_users(&mut self) -> Vec<PDUSession> {
        self.connected_users.drain(..).collect()
    }

    pub fn update_connected_users(&mut self) -> Vec<PDUSession> {
        self.connected_users.iter_mut().for_each(|pdu_session| {
            pdu_session.update_user_position();
        });

        let mut res: Vec<PDUSession> = Vec::new();
        let tmp: Vec<PDUSession> = self.connected_users.drain(..).collect();
        tmp.into_iter().for_each(|pdu_session| {
            if self.contains(pdu_session.user()) {
                self.connected_users.push(pdu_session);
            } else {
                res.push(pdu_session);
            }
        });
        res
    }

    pub fn get_current_connected_users(&self) -> Vec<&PDUSession> {
        self.connected_users.iter().collect()
    }

    pub fn get_current_connected_users_mut(&mut self) -> Vec<&mut PDUSession> {
        self.connected_users.iter_mut().collect()
    }

    pub fn connect_user(&mut self, user: PDUSession) {
        self.connected_users.push(user);
    }

    pub fn connect_users(&mut self, mut users: Vec<PDUSession>) {
        self.connected_users.append(&mut users);
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_position(&self) -> Point {
        self.position
    }
}

impl Contains<User> for Ran {
    fn contains(&self, rhs: &User) -> bool {
        self.position.euclidean_distance(&rhs.current_pos()).abs() <= self.radius
    }
}

impl Serialize for Ran {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Ran", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("x", &self.position.x())?;
        state.serialize_field("y", &self.position.y())?;
        state.serialize_field("radius", &self.radius)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use geo::Point;
    use std::net::{IpAddr, Ipv4Addr};

    use super::*;

    #[test]
    fn connect_users() {
        let position = Point::new(0.5, 0.5);
        let mut ran = Ran::new(1, position, 0.5);
        let pdu_sessions: Vec<PDUSession> = (0..32)
            .map(|i| {
                PDUSession::new(
                    User::new(i, position, 1., &(-50.0..50.)),
                    IpAddr::V4(Ipv4Addr::LOCALHOST),
                    &ran,
                )
            })
            .collect();
        ran.connect_users(pdu_sessions.clone());
        assert_eq!(ran.connected_users, pdu_sessions);
    }

    #[test]
    fn connect_user() {
        let position = Point::new(0.5, 0.5);
        let mut ran = Ran::new(1, position, 0.5);
        let pdu_sessions: Vec<PDUSession> = (0..32)
            .map(|i| {
                PDUSession::new(
                    User::new(i, position, 1., &(-50.0..50.)),
                    IpAddr::V4(Ipv4Addr::LOCALHOST),
                    &ran,
                )
            })
            .collect();
        pdu_sessions
            .clone()
            .into_iter()
            .for_each(|pdu_session| ran.connect_user(pdu_session));
        assert_eq!(ran.connected_users, pdu_sessions);
    }

    #[test]
    fn contains() {
        let mut position = Point::new(0.5, 0.5);
        let ran = Ran::new(1, position, 0.5);
        let mut usr = User::new(0, position, 1.0, &(-50.0..50.));

        let mut res = ran.contains(&usr);
        assert!(res);

        position = Point::new(1.5, 1.5);
        usr = User::new(0, position, 1.0, &(-50.0..50.));

        res = ran.contains(&usr);
        assert!(!res);
    }

    #[test]
    fn get_connected_users() {
        let position = Point::new(0.5, 0.5);
        let mut ran = Ran::new(1, position, 0.5);
        let pdu_sessions: Vec<PDUSession> = (0..32)
            .map(|i| {
                PDUSession::new(
                    User::new(i, position, 1., &(-50.0..50.0)),
                    IpAddr::V4(Ipv4Addr::LOCALHOST),
                    &ran,
                )
            })
            .collect();
        ran.connect_users(pdu_sessions.clone());
        let all_pdu_sessions = ran.get_connected_users();
        assert_eq!(all_pdu_sessions, pdu_sessions);
    }

    #[test]
    fn get_current_connected_users() {
        let position = Point::new(0.5, 0.5);
        let mut ran = Ran::new(1, position, 0.5);
        let pdu_sessions: Vec<PDUSession> = (0..32)
            .map(|i| {
                PDUSession::new(
                    User::new(i, position, 1., &(-50.0..50.)),
                    IpAddr::V4(Ipv4Addr::LOCALHOST),
                    &ran,
                )
            })
            .collect();
        ran.connect_users(pdu_sessions);
        let all_pdu_sessions = ran.get_current_connected_users();
        assert_eq!(all_pdu_sessions.len(), 32);
    }
}
