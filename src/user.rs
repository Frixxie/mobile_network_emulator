use rand::prelude::*;
use std::{error::Error, fmt::Display, iter::repeat, net::IpAddr};

use geo::{MultiPoint, Point};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PDUSession {
    ip_address: IpAddr,
}

#[derive(Debug)]
pub struct UserError {
    msg: String,
}

impl UserError {
    pub fn new(msg: &str) -> Self {
        UserError {
            msg: msg.to_string(),
        }
    }
}

impl Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.msg)
    }
}

impl Error for UserError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UserState {
    Connected(PDUSession),
    InReach,
    OutOfReach,
    Unknown,
}

impl Display for UserState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            UserState::Connected(s) => format!("Connected, IP: {}", s.ip_address),
            UserState::InReach => "InReach".to_string(),
            UserState::OutOfReach => "OutOfReach".to_string(),
            UserState::Unknown => "Unknown".to_string(),
        };
        f.write_str(&res)
    }
}

#[derive(Clone, Debug)]
pub struct User {
    pub gpsi: u32,
    pub state: UserState,
    current_pos: usize,
    trail: Option<MultiPoint>,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let point = self.current_pos().unwrap().x_y();
        f.write_str(
            &format!("{}, {}, ({}, {})", self.gpsi, self.state, point.0, point.1).to_string(),
        )
    }
}

impl User {
    pub fn new(gpsi: u32) -> Self {
        User {
            gpsi,
            state: UserState::Unknown,
            current_pos: 0,
            trail: None,
        }
    }

    pub fn connect_mobile_network(&mut self, pdu_session: PDUSession) -> Result<IpAddr, UserError> {
        if self.state == UserState::InReach {
            self.state = UserState::Connected(pdu_session.clone());
            return Ok(pdu_session.ip_address);
        }
        Err(UserError::new("UE is not in reach"))
    }

    pub fn add_trail(&mut self, points: Vec<Point>) {
        self.trail = Some(MultiPoint::new(points));
        self.current_pos = 0
    }

    pub fn current_pos(&self) -> Option<Point> {
        match &self.trail {
            Some(t) => t.iter().nth(self.current_pos).copied(),
            None => None,
        }
    }

    pub fn generate_user_trail(bounds: (f64, f64), start_pos: Point, length: usize) -> MultiPoint {
        let mut rng = rand::thread_rng();
        let mut res = Vec::new();
        res.push(start_pos);
        for i in 0..length - 1 {
            let diff = (rng.gen_range(-1.0..1.), rng.gen_range(-1.0..1.));
            let point = Point::new(
                (res[i].x() + diff.0) % bounds.0,
                (res[i].y() + diff.1) % bounds.1,
            );
            res.push(point)
        }
        MultiPoint(res)
    }

    pub fn move_next(&mut self) -> usize {
        self.current_pos += 1;
        self.current_pos
    }

    pub fn move_prev(&mut self) -> usize {
        self.current_pos -= 1;
        self.current_pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_trail() {
        let trail = User::generate_user_trail((100., 100.), (50., 50.).into(), 1 << 7);
        assert_eq!(trail.iter().count(), 1 << 7);
    }
}
