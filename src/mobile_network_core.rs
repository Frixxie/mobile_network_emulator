use std::{
    net::{IpAddr, Ipv4Addr},
    time::{SystemTime, UNIX_EPOCH},
};

use geo::{Contains, Point};

use crate::{
    mobile_network_core_event::{
        AccuracyFulfillmentIndicator, CivicAddress, Event, EventSubscriber, GeographicArea,
        InterfaceIndication, LdrType, LocationInfo, MinorLocationQoS, MobileNetworkCoreEvent,
        PdnConnectionInformation, PdnConnectionStatus, PdnType, PositioningMethod,
    },
    pdu_session::PDUSession,
    ran::Ran,
    user::User,
};

pub struct MobileNetworkCore {
    rans: Vec<Ran>,
    orphans: Vec<User>,
    available_ip_addresses: Vec<IpAddr>,
    events: Vec<MobileNetworkCoreEvent>,
    event_subscribers: Vec<EventSubscriber>,
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
                    self.events.push(Self::create_location_reporting_event(
                        &ran.get_id().to_string(),
                        pdu_session.user().current_pos(),
                        LdrType::EnteringIntoArea,
                    ));
                    ran.connect_user(pdu_session);
                    let v4addr = match ip_address {
                        IpAddr::V4(v4addr) => v4addr,
                        _ => unreachable!(),
                    };
                    self.events.push(Self::create_pdn_connection_event(v4addr));
                    break;
                }
            }
        }
        self.orphans = tmp_orphans;
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
                let v4addr = match ip_address {
                    IpAddr::V4(v4addr) => v4addr,
                    _ => unreachable!(),
                };
                // TODO: Fix this loop
                self.events.push(Self::create_location_reporting_event(
                    "Ups ran id is supposed to be here",
                    user.current_pos(),
                    LdrType::LeavingFromArea,
                ));
                self.events.push(Self::release_pdn_connection_event(v4addr));
                self.available_ip_addresses.push(ip_address);
                user
            })
            .collect();
        self.orphans.append(&mut new_orphans);
    }

    pub fn generate_location_events(&mut self) {
        let connected_users: Vec<&PDUSession> = self
            .rans
            .iter()
            .flat_map(|ran| ran.get_current_connected_users())
            .collect();
        for ran in self.rans.iter() {
            for pdu_session in connected_users.iter() {
                if ran.contains(pdu_session.user()) {
                    self.events.push(Self::create_location_reporting_event(
                        &ran.get_id().to_string(),
                        pdu_session.user().current_pos(),
                        LdrType::Motion,
                    ));
                }
            }
        }
    }

    pub fn get_rans(&self) -> Vec<&Ran> {
        self.rans.iter().collect()
    }

    pub fn get_connected_users(&self) -> Vec<&PDUSession> {
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
            .iter()
            .map(|pdu_session| pdu_session.user())
            .chain(self.orphans.iter())
            .collect()
    }

    pub fn add_subscriber(&mut self, event_subscriber: EventSubscriber) {
        self.event_subscribers.push(event_subscriber)
    }

    pub fn get_subscribers(&self) -> Vec<&EventSubscriber> {
        self.event_subscribers.iter().collect()
    }

    pub fn publish_events(&self) {
        todo!();
    }

    pub fn get_events(&self) -> Vec<&MobileNetworkCoreEvent> {
        self.events.iter().collect()
    }

    fn create_location_reporting_event(
        ran_id: &str,
        user_pos: Point,
        ldr_type: LdrType,
    ) -> MobileNetworkCoreEvent {
        let geophraphical_location = GeographicArea::Point(user_pos);
        let loc_info = LocationInfo::new(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ran_id.to_string(),
            geophraphical_location,
            CivicAddress {},
            vec![PositioningMethod::CellId],
            AccuracyFulfillmentIndicator::RequestedAccuracyFulfilled,
            1.0,
            ldr_type,
            MinorLocationQoS::new(1.0, 1.0),
        );
        MobileNetworkCoreEvent::new(Event::LocationReporting(loc_info))
    }

    fn create_pdn_connection_event(ipv4_addr: Ipv4Addr) -> MobileNetworkCoreEvent {
        MobileNetworkCoreEvent::new(Event::PdnConnectionEvent(PdnConnectionInformation::new(
            PdnConnectionStatus::Created,
            PdnType::Ipv4,
            InterfaceIndication::ExposureFunction,
            ipv4_addr,
        )))
    }

    fn release_pdn_connection_event(ipv4_addr: Ipv4Addr) -> MobileNetworkCoreEvent {
        MobileNetworkCoreEvent::new(Event::PdnConnectionEvent(PdnConnectionInformation::new(
            PdnConnectionStatus::Released,
            PdnType::Ipv4,
            InterfaceIndication::ExposureFunction,
            ipv4_addr,
        )))
    }
}

#[cfg(test)]
mod tests {

    use std::net::Ipv4Addr;

    use super::*;
    use geo::Point;

    #[test]
    fn try_connect_orphans() {
        //setup
        let position = Point::new(0.5, 0.5);
        let ran = Ran::new(1, position, 0.5);
        let usr = User::new(0, position, 1.0, &(-50.0..50.));
        let ip_addesses = vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))];
        let mut mn = MobileNetworkCore::new(vec![ran], vec![usr], ip_addesses);

        //execute
        mn.try_connect_orphans();

        //verify
        assert_eq!(mn.orphans.len(), 0);
    }

    #[test]
    fn update_user_positions() {
        //setup
        let position = Point::new(0.5, 0.5);
        let ran = Ran::new(1, position, 0.5);
        let usr = User::new(0, position, 1.0, &(-50.0..50.));
        let ip_addesses = vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))];
        let mut mn = MobileNetworkCore::new(vec![ran], vec![usr], ip_addesses);

        //execute
        mn.update_user_positions();

        //verify
        assert_eq!(mn.orphans.len(), 1);
    }
}
