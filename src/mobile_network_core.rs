use std::net::IpAddr;

use geo::{Contains, Point};

use crate::{
    mobule_network_core_event::MobileNetworkCoreEvent, pdu_session::PDUSession, ran::Ran,
    subscription::Subscription, user::User,
};

pub struct MobileNetworkCore {
    rans: Vec<Ran>,
    orphans: Vec<User>,
    available_ip_addresses: Vec<IpAddr>,
    events: Vec<MobileNetworkCoreEvent>,
    event_subscribers: Vec<Subscription>,
}

impl MobileNetworkCore {
    pub fn new(rans: Vec<Ran>, orphans: Vec<User>, ip_addesses: Vec<IpAddr>) -> Self {
        MobileNetworkCore {
            rans,
            orphans,
            available_ip_addresses: ip_addesses,
            events: Vec::new(),
            event_subscribers: Vec::new(),
        }
    }

    /// Updates all users positions and places new orphans in orphans.
    pub fn update_user_positions(&mut self) {
        self.orphans.iter_mut().for_each(|user| {
            user.next_pos();
        });
        let mut new_orphans = self
            .rans
            .iter_mut()
            .flat_map(|ran| ran.update_connected_users())
            .map(|pdu_session| {
                let (user, ip_address) = pdu_session.release();
                self.available_ip_addresses.push(ip_address);
                user
            })
            .collect();
        self.orphans.append(&mut new_orphans);
    }

    pub fn try_connect_orphans(&mut self) {
        let mut tmp_orphans = Vec::new();
        for user in self.orphans.drain(..) {
            tmp_orphans.push(user);
            for ran in self.rans.iter_mut() {
                if ran.contains(tmp_orphans.last().unwrap()) {
                    let ip_address = match self.available_ip_addresses.pop() {
                        Some(ip_address) => ip_address,
                        None => unreachable!(),
                    };
                    let pdu_session = PDUSession::new(tmp_orphans.pop().unwrap(), ip_address);
                    ran.connect_user(pdu_session);
                    break;
                }
            }
        }
        self.orphans = tmp_orphans;
    }

    pub fn get_rans(&self) -> Vec<&Ran> {
        self.rans.iter().map(|ran| ran).collect()
    }

    pub fn get_connected_users(&self) -> Vec<&User> {
        self.rans
            .iter()
            .flat_map(|ran| ran.get_current_connected_users())
            .collect()
    }

    pub fn get_mut_pdu_sessions(&mut self) -> Vec<(&Point, Vec<&mut PDUSession>)> {
        self.rans
            .iter_mut()
            .map(|ran| ran.get_mut_current_connected_users())
            .collect()
    }

    pub fn get_all_users(&self) -> Vec<&User> {
        self.get_connected_users()
            .into_iter()
            .chain(self.orphans.iter())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::iter::repeat;
    use std::net::Ipv4Addr;

    use super::*;
    use geo::{MultiPoint, Point};

    #[test]
    fn try_connect_orphans() {
        //setup
        let position = Point::new(0.5, 0.5);
        let ran = Ran::new(position, 0.5);
        let mut usr = User::new(0);
        usr.add_path(MultiPoint::new(vec![
            Point::new(0.5, 0.5),
            Point::new(1.1, 1.1),
        ]));
        let ip_addesses = vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))];
        let mut mn = MobileNetworkCore::new(vec![ran], vec![usr], ip_addesses);

        //execute
        mn.try_connect_orphans();

        //verify
        assert_eq!(mn.orphans.len(), 0);
    }

    #[test]
    fn update_user_posititons() {
        //setup
        let position = Point::new(0.5, 0.5);
        let ran = Ran::new(position, 0.5);
        let mut users: Vec<User> = (0..20).map(User::new).collect();
        for user in users.iter_mut().take(10) {
            user.add_path(MultiPoint::new(vec![
                Point::new(0.5, 0.5),
                Point::new(0.8, 0.8),
            ]));
        }
        for user in users.iter_mut().skip(10) {
            user.add_path(MultiPoint::new(vec![
                Point::new(0.5, 0.5),
                Point::new(1.1, 1.1),
            ]));
        }

        let ip_addresses: Vec<IpAddr> = repeat(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
            .take(20)
            .collect();
        let mut mn = MobileNetworkCore::new(vec![ran], users, ip_addresses);

        //execute and verify
        mn.try_connect_orphans();
        assert_eq!(mn.orphans.len(), 0);
        mn.update_user_positions();
        assert_eq!(mn.orphans.len(), 10);
        mn.try_connect_orphans();
        assert_eq!(mn.orphans.len(), 10);
    }

    #[test]
    fn get_all_users() {
        //setup
        let position = Point::new(0.5, 0.5);
        let ran = Ran::new(position, 0.5);
        let mut users: Vec<User> = (0..20).map(User::new).collect();
        for user in users.iter_mut().take(10) {
            user.add_path(MultiPoint::new(vec![
                Point::new(0.5, 0.5),
                Point::new(0.8, 0.8),
            ]));
        }
        for user in users.iter_mut().skip(10) {
            user.add_path(MultiPoint::new(vec![
                Point::new(0.5, 0.5),
                Point::new(1.1, 1.1),
            ]));
        }

        let ip_addresses: Vec<IpAddr> = repeat(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
            .take(20)
            .collect();
        let mut mn = MobileNetworkCore::new(vec![ran], users, ip_addresses);

        //execute and verify
        mn.try_connect_orphans();
        assert_eq!(mn.orphans.len(), 0);
        let all_users = mn.get_all_users();
        assert_eq!(all_users.len(), 20);

        mn.update_user_positions();
        let all_users = mn.get_all_users();
        assert_eq!(all_users.len(), 20);

        mn.try_connect_orphans();
        assert_eq!(mn.orphans.len(), 10);
        let all_users = mn.get_all_users();
        assert_eq!(all_users.len(), 20);
    }

    #[test]
    fn get_connected_users() {
        //setup
        let position = Point::new(0.5, 0.5);
        let ran = Ran::new(position, 0.5);
        let mut users: Vec<User> = (0..20).map(User::new).collect();
        for user in users.iter_mut().take(10) {
            user.add_path(MultiPoint::new(vec![
                Point::new(0.5, 0.5),
                Point::new(0.8, 0.8),
            ]));
        }
        for user in users.iter_mut().skip(10) {
            user.add_path(MultiPoint::new(vec![
                Point::new(0.5, 0.5),
                Point::new(1.1, 1.1),
            ]));
        }

        let ip_addresses: Vec<IpAddr> = repeat(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
            .take(20)
            .collect();
        let mut mn = MobileNetworkCore::new(vec![ran], users, ip_addresses);

        //execute and verify
        mn.try_connect_orphans();
        assert_eq!(mn.orphans.len(), 0);
        let connected_users = mn.get_connected_users();
        assert_eq!(connected_users.len(), 20);

        mn.update_user_positions();

        mn.try_connect_orphans();
        assert_eq!(mn.orphans.len(), 10);
        let connected_users = mn.get_connected_users();
        assert_eq!(connected_users.len(), 10);
    }
}
