use std::net::IpAddr;

// pub struct MonitoringEventReport {
//     imeiChange: Option<AssosiationType>
//     externalId: Option<Vec<ExternalId>
//     ideStatusInfo: Option<IdelStatusInfo>

// }

use geo::Contains;

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

    /// Updates all users positions and places them in orphans.
    pub fn update_user_positions(&mut self) {
        self.orphans = self
            .rans
            .iter_mut()
            .map(|ran| ran.get_connected_users())
            .flatten()
            .map(|pdu_session| {
                let (user, ip_address) = pdu_session.release();
                self.available_ip_addresses.push(ip_address);
                user
            })
            .chain(self.orphans.drain(..))
            .map(|mut user| {
                user.next_pos();
                user
            })
            .collect()
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
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use geo::{MultiPoint, Point, Rect};

    use super::*;

    #[test]
    fn try_connect_orphans() {
        //setup
        let ran = Ran::new(Rect::new(Point::new(0.0, 0.0), Point::new(1., 1.)));
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
        assert_eq!(mn.orphans.iter().count(), 0);
    }

    #[test]
    fn update_user_posititons() {
        //setup
        let ran = Ran::new(Rect::new(Point::new(0.0, 0.0), Point::new(1., 1.)));
        let mut usr = User::new(0);
        usr.add_path(MultiPoint::new(vec![
            Point::new(0.5, 0.5),
            Point::new(1.1, 1.1),
        ]));
        let ip_addesses = vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))];
        let mut mn = MobileNetworkCore::new(vec![ran], vec![usr], ip_addesses);

        //execute and verify
        mn.try_connect_orphans();
        assert_eq!(mn.orphans.iter().count(), 0);
        mn.update_user_positions();
        assert_eq!(mn.orphans.iter().count(), 1);
        mn.try_connect_orphans();
        assert_eq!(mn.orphans.iter().count(), 1);
    }
}
