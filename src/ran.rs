use geo::{Contains, Rect};

use crate::{pdu_session::PDUSession, user::User};

pub struct Ran {
    cell: Rect,
    connected_users: Vec<PDUSession>,
}

impl Ran {
    pub fn new(cell: Rect) -> Self {
        Ran {
            cell,
            connected_users: Vec::new(),
        }
    }

    pub fn get_connected_users(&mut self) -> Vec<PDUSession> {
        self.connected_users.drain(..).collect()
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
        self.cell.contains(&user_pos)
    }
}

#[cfg(test)]
mod tests {
    use geo::{MultiPoint, Point};
    use std::net::{IpAddr, Ipv4Addr};

    use super::*;

    #[test]
    fn connect_users() {
        let mut ran = Ran::new(Rect::new(Point::new(0.0, 0.0), Point::new(1., 1.)));
        let pdu_sessions: Vec<PDUSession> = (0..32)
            .into_iter()
            .map(|i| PDUSession::new(User::new(i), IpAddr::V4(Ipv4Addr::LOCALHOST)))
            .collect();
        ran.connect_users(pdu_sessions.clone());
        assert_eq!(ran.connected_users, pdu_sessions);
    }

    #[test]
    fn contains() {
        let ran = Ran::new(Rect::new(Point::new(0.0, 0.0), Point::new(1., 1.)));
        let mut usr = User::new(0);
        usr.add_path(MultiPoint::new(vec![
            Point::new(0.5, 0.5),
            Point::new(1.1, 1.1),
        ]));

        let mut res = ran.contains(&usr);
        assert_eq!(res, true);

        usr.next_pos();

        res = ran.contains(&usr);
        assert_eq!(res, false);
    }
}
