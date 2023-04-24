use geo::Contains;
use geo::EuclideanDistance;
use geo::Point;
use serde::ser::SerializeStruct;
use serde::Serialize;

use crate::{pdu_session::PDUSession, user::User};

#[derive(Debug, Clone)]
pub struct Ran {
    position: Point,
    radius: f64,
    connected_users: Vec<PDUSession>,
}

impl Ran {
    pub fn new(position: Point, radius: f64) -> Self {
        Ran {
            position,
            radius,
            connected_users: Vec::new(),
        }
    }

    pub fn get_connected_users(&mut self) -> Vec<PDUSession> {
        self.connected_users.drain(..).collect()
    }

    pub fn update_connected_users(&mut self) -> Vec<PDUSession> {
        self.connected_users.iter_mut().for_each(|pdu_session| {
            match pdu_session.update_user_position() {
                Some(_) => (),
                None => panic!("PDU Session user is missing trail"),
            }
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

    pub fn get_current_connected_users(&self) -> Vec<&User> {
        self.connected_users
            .iter()
            .map(|pdu_session| pdu_session.user())
            .collect()
    }

    pub fn connect_user(&mut self, user: PDUSession) {
        self.connected_users.push(user);
    }

    pub fn connect_users(&mut self, mut users: Vec<PDUSession>) {
        self.connected_users.append(&mut users);
    }
}

impl Contains<User> for Ran {
    fn contains(&self, rhs: &User) -> bool {
        let user_pos = match rhs.current_pos() {
            Some(pos) => pos,
            None => panic!("User should have position before calling this function"),
        };
        self.position.euclidean_distance(&user_pos).abs() <= self.radius
    }
}

impl Serialize for Ran {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Ran", 3)?;
        state.serialize_field("x", &self.position.x())?;
        state.serialize_field("y", &self.position.y())?;
        state.serialize_field("radius", &self.radius)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use geo::{MultiPoint, Point};
    use std::net::{IpAddr, Ipv4Addr};

    use super::*;

    #[test]
    fn connect_users() {
        let position = Point::new(0.5, 0.5);
        let mut ran = Ran::new(position, 0.5);
        let pdu_sessions: Vec<PDUSession> = (0..32)
            .map(|i| PDUSession::new(User::new(i), IpAddr::V4(Ipv4Addr::LOCALHOST)))
            .collect();
        ran.connect_users(pdu_sessions.clone());
        assert_eq!(ran.connected_users, pdu_sessions);
    }

    #[test]
    fn connect_user() {
        let position = Point::new(0.5, 0.5);
        let mut ran = Ran::new(position, 0.5);
        let pdu_sessions: Vec<PDUSession> = (0..32)
            .map(|i| PDUSession::new(User::new(i), IpAddr::V4(Ipv4Addr::LOCALHOST)))
            .collect();
        pdu_sessions
            .clone()
            .into_iter()
            .for_each(|pdu_session| ran.connect_user(pdu_session));
        assert_eq!(ran.connected_users, pdu_sessions);
    }

    #[test]
    #[should_panic]
    fn contains_should_panic() {
        let position = Point::new(0.5, 0.5);
        let ran = Ran::new(position, 0.5);
        let usr = User::new(0);
        ran.contains(&usr);
    }

    #[test]
    fn contains() {
        let position = Point::new(0.5, 0.5);
        let ran = Ran::new(position, 0.5);
        let mut usr = User::new(0);

        usr.add_path(MultiPoint::new(vec![
            Point::new(0.5, 0.5),
            Point::new(1.1, 1.1),
        ]));

        let mut res = ran.contains(&usr);
        assert!(res);

        usr.next_pos();

        res = ran.contains(&usr);
        assert!(!res);
    }

    #[test]
    fn get_connected_users() {
        let position = Point::new(0.5, 0.5);
        let mut ran = Ran::new(position, 0.5);
        let pdu_sessions: Vec<PDUSession> = (0..32)
            .map(|i| PDUSession::new(User::new(i), IpAddr::V4(Ipv4Addr::LOCALHOST)))
            .collect();
        ran.connect_users(pdu_sessions.clone());
        let all_pdu_sessions = ran.get_connected_users();
        assert_eq!(all_pdu_sessions, pdu_sessions);
    }

    #[test]
    fn get_current_connected_users() {
        let position = Point::new(0.5, 0.5);
        let mut ran = Ran::new(position, 0.5);
        let pdu_sessions: Vec<PDUSession> = (0..32)
            .map(|i| PDUSession::new(User::new(i), IpAddr::V4(Ipv4Addr::LOCALHOST)))
            .collect();
        ran.connect_users(pdu_sessions);
        let all_pdu_sessions = ran.get_current_connected_users();
        assert_eq!(all_pdu_sessions.len(), 32);
    }
}
