use std::{
    net::{IpAddr, Ipv4Addr},
    time::{SystemTime, UNIX_EPOCH},
};

use geo::{Contains, Point};
use log::info;
use mobile_network_core_event::{
    AccuracyFulfillmentIndicator, CivicAddress, Event,
    EventKind::{LocationReporting, PdnConnectionEvent},
    GeographicArea, InterfaceIndication, LdrType, LocationInfo, MinorLocationQoS,
    MobileNetworkCoreEvent, PdnConnectionInformation, PdnConnectionStatus, PdnType,
    PositioningMethod,
};
use mongodb::{Collection, Database};
use rand::seq::SliceRandom;

use crate::{
    application::Application,
    network::{Network, NetworkLogEntry},
    pdu_session::PDUSession,
    ran::Ran,
    user::User,
};

pub struct MobileNetworkCore {
    rans: Vec<Ran>,
    orphans: Vec<User>,
    available_ip_addresses: Vec<IpAddr>,
}

impl MobileNetworkCore {
    pub fn new(rans: Vec<Ran>, orphans: Vec<User>, ip_addesses: Vec<IpAddr>) -> Self {
        MobileNetworkCore {
            rans,
            orphans,
            available_ip_addresses: ip_addesses,
        }
    }

    pub async fn try_connect_orphans(&mut self, database: &Database) {
        let collection: Collection<MobileNetworkCoreEvent> = database.collection("Events");
        let mut new_events: Vec<MobileNetworkCoreEvent> = Vec::new();
        let mut tmp_orphans = Vec::new();
        for user in self.orphans.drain(..) {
            tmp_orphans.push(user);
            for ran in self.rans.iter_mut() {
                if ran.contains(tmp_orphans.last().unwrap()) {
                    let ip_address = match self.available_ip_addresses.pop() {
                        Some(ip_address) => ip_address,
                        None => unreachable!(),
                    };
                    let pdu_session = PDUSession::new(tmp_orphans.pop().unwrap(), ip_address, ran);
                    new_events.push(Self::create_location_reporting_event(
                        &ran.get_id().to_string(),
                        pdu_session.user().current_pos(),
                        LdrType::EnteringIntoArea,
                        pdu_session.user().get_id(),
                    ));
                    let v4addr = match ip_address {
                        IpAddr::V4(v4addr) => v4addr,
                        _ => unreachable!(),
                    };
                    new_events.push(Self::create_pdn_connection_event(
                        v4addr,
                        pdu_session.user().get_id(),
                    ));
                    ran.connect_user(pdu_session);
                    break;
                }
            }
        }
        self.orphans = tmp_orphans;
        if !new_events.is_empty() {
            collection.insert_many(new_events, None).await.unwrap();
        }
    }

    /// Updates all users positions and places new orphans in orphans.
    pub async fn update_user_positions(&mut self, database: &Database) {
        let collection: Collection<MobileNetworkCoreEvent> = database.collection("Events");
        let mut new_events: Vec<MobileNetworkCoreEvent> = Vec::new();
        self.orphans.iter_mut().for_each(|user| {
            user.next_pos();
        });
        let mut new_orphans = Vec::new();
        for ran_index in 0..self.rans.len() {
            let pdu_sessions = self.rans[ran_index].update_connected_users();
            'next_pdu_session: for pdu_session in pdu_sessions {
                for ran in self.rans.iter_mut() {
                    if ran.contains(pdu_session.user()) {
                        info!(
                            "user with id {} and ip {} handed over to {}",
                            pdu_session.user(),
                            pdu_session.ip().to_string(),
                            ran.get_id()
                        );
                        ran.connect_user(pdu_session);
                        continue 'next_pdu_session;
                    }
                }
                let (user, ip_address) = pdu_session.release();
                let v4addr = match ip_address {
                    IpAddr::V4(v4addr) => v4addr,
                    _ => unreachable!(),
                };
                new_events.push(Self::create_location_reporting_event(
                    format!("{}", self.rans[ran_index].get_id()).as_str(),
                    user.current_pos(),
                    LdrType::LeavingFromArea,
                    user.get_id(),
                ));
                new_events.push(Self::release_pdn_connection_event(v4addr, user.get_id()));
                self.available_ip_addresses.push(ip_address);
                new_orphans.push(user);
            }
        }
        self.orphans.append(&mut new_orphans);
        if !new_events.is_empty() {
            collection.insert_many(new_events, None).await.unwrap();
        }
    }

    pub async fn generate_location_events(&self, database: &Database) {
        let collection: Collection<MobileNetworkCoreEvent> = database.collection("Events");
        let all_events: Vec<MobileNetworkCoreEvent> = self
            .get_connected_users()
            .iter()
            .map(|pdu_session| {
                Self::create_location_reporting_event(
                    &pdu_session.get_ran().get_id().to_string(),
                    pdu_session.user().current_pos(),
                    LdrType::Motion,
                    pdu_session.user().get_id(),
                )
            })
            .collect();
        if !all_events.is_empty() {
            collection.insert_many(all_events, None).await.unwrap();
        }
    }

    pub async fn use_some_applications(&self, network: &mut Network, database: &Database) {
        let collection: Collection<NetworkLogEntry> = database.collection("NetworkLog");
        let connected_users = self.get_connected_users();
        let some_users =
            connected_users.choose_multiple(&mut rand::thread_rng(), connected_users.len() / 2);
        let applications: Vec<Application> =
            network.get_applictions().into_iter().cloned().collect();

        let mut network_logs = Vec::new();
        for user in some_users {
            let application = match applications.choose(&mut rand::thread_rng()) {
                Some(application) => application,
                None => {
                    info!("The application list is empty there is no application accessed for user with ip: {}", user.ip());
                    break;
                }
            };
            let res = network
                .use_application(user, application, &user.get_ran().get_position())
                .unwrap();
            network_logs.push(res);
        }
        let avg_point: Point = some_users.cloned()
            .map(|pdu_session| pdu_session.user().current_pos())
            .reduce(|acc, point| acc + point)
            .unwrap()
            / some_users.len() as f64;
        info!("avg point {},{}", avg_point.x(), avg_point.y());
        if !network_logs.is_empty() {
            collection.insert_many(network_logs, None).await.unwrap();
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

    pub fn get_all_users(&self) -> Vec<&User> {
        self.get_connected_users()
            .iter()
            .map(|pdu_session| pdu_session.user())
            .chain(self.orphans.iter())
            .collect()
    }

    fn create_location_reporting_event(
        ran_id: &str,
        user_pos: Point,
        ldr_type: LdrType,
        user_id: u32,
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
        MobileNetworkCoreEvent::new(
            Event::LocationReporting(loc_info),
            LocationReporting,
            user_id,
        )
    }

    fn create_pdn_connection_event(ipv4_addr: Ipv4Addr, user_id: u32) -> MobileNetworkCoreEvent {
        MobileNetworkCoreEvent::new(
            Event::PdnConnectionEvent(PdnConnectionInformation::new(
                PdnConnectionStatus::Created,
                PdnType::Ipv4,
                InterfaceIndication::ExposureFunction,
                ipv4_addr,
            )),
            PdnConnectionEvent,
            user_id,
        )
    }

    fn release_pdn_connection_event(ipv4_addr: Ipv4Addr, user_id: u32) -> MobileNetworkCoreEvent {
        MobileNetworkCoreEvent::new(
            Event::PdnConnectionEvent(PdnConnectionInformation::new(
                PdnConnectionStatus::Released,
                PdnType::Ipv4,
                InterfaceIndication::ExposureFunction,
                ipv4_addr,
            )),
            PdnConnectionEvent,
            user_id,
        )
    }
}

#[cfg(test)]
mod tests {

    //#[test]
    //fn try_connect_orphans() {
    //    //setup
    //    let position = Point::new(0.5, 0.5);
    //    let ran = Ran::new(1, position, 0.5);
    //    let usr = User::new(0, position, 1.0, &(-50.0..50.));
    //    let ip_addesses = vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))];
    //    let mut mn = MobileNetworkCore::new(vec![ran], vec![usr], ip_addesses);

    //    //execute
    //    mn.try_connect_orphans();

    //    //verify
    //    assert_eq!(mn.orphans.len(), 0);
    //}

    //#[test]
    //fn update_user_positions() {
    //    //setup
    //    let position = Point::new(0.5, 0.5);
    //    let ran = Ran::new(1, position, 0.5);
    //    let usr = User::new(0, position, 1.0, &(-50.0..50.));
    //    let ip_addesses = vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))];
    //    let mut mn = MobileNetworkCore::new(vec![ran], vec![usr], ip_addesses);

    //    //execute
    //    mn.update_user_positions();

    //    //verify
    //    assert_eq!(mn.orphans.len(), 1);
    //}

    //#[test]
    //fn generate_location_events() {
    //    //setup
    //    let position = Point::new(0.5, 0.5);
    //    let ran = Ran::new(1, position, 0.5);
    //    let usr = User::new(0, position, 1.0, &(-50.0..50.));
    //    let ip_addesses = vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))];
    //    let mut mn = MobileNetworkCore::new(vec![ran], vec![usr], ip_addesses);

    //    //execute
    //    mn.try_connect_orphans();
    //    mn.generate_location_events();

    //    //verify
    //    assert_eq!(mn.events.len(), 3);
    //}

    //#[test]
    //fn get_rans() {
    //    //setup
    //    let position = Point::new(0.5, 0.5);
    //    let ran = Ran::new(1, position, 0.5);
    //    let usr = User::new(0, position, 1.0, &(-50.0..50.));
    //    let ip_addesses = vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))];
    //    let mn = MobileNetworkCore::new(vec![ran], vec![usr], ip_addesses);

    //    //execute
    //    let rans = mn.get_rans();

    //    //verify
    //    assert_eq!(rans.len(), 1);
    //}

    //#[test]
    //fn get_users() {
    //    //setup
    //    let position = Point::new(0.5, 0.5);
    //    let ran = Ran::new(1, position, 0.5);
    //    let usr = User::new(0, position, 1.0, &(-50.0..50.));
    //    let ip_addesses = vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))];
    //    let mn = MobileNetworkCore::new(vec![ran], vec![usr], ip_addesses);

    //    //execute
    //    let users = mn.get_all_users();

    //    //verify
    //    assert_eq!(users.len(), 1);
    //}

    //#[test]
    //fn get_connected_users() {
    //    //setup
    //    let position = Point::new(0.5, 0.5);
    //    let ran = Ran::new(1, position, 0.5);
    //    let usr = User::new(0, position, 1.0, &(-50.0..50.));
    //    let ip_addesses = vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))];
    //    let mut mn = MobileNetworkCore::new(vec![ran], vec![usr], ip_addesses);
    //    mn.try_connect_orphans();

    //    //execute
    //    let users = mn.get_connected_users();

    //    //verify
    //    assert_eq!(users.len(), 1);
    //}

    //#[test]
    //fn add_subscriber() {
    //    //setup
    //    let position = Point::new(0.5, 0.5);
    //    let ran = Ran::new(1, position, 0.5);
    //    let usr = User::new(0, position, 1.0, &(-50.0..50.));
    //    let ip_addesses = vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))];
    //    let mut mn = MobileNetworkCore::new(vec![ran], vec![usr], ip_addesses);

    //    let event_subscriber = EventSubscriber::new(
    //        Url::parse("http://test:8080").unwrap(),
    //        PdnConnectionEvent,
    //        vec![1, 2, 3],
    //    );

    //    //execute
    //    mn.add_subscriber(event_subscriber);

    //    //verify
    //    assert_eq!(mn.event_subscribers.len(), 1);
    //}

    //#[test]
    //fn get_subscribers() {
    //    //setup
    //    let position = Point::new(0.5, 0.5);
    //    let ran = Ran::new(1, position, 0.5);
    //    let usr = User::new(0, position, 1.0, &(-50.0..50.));
    //    let ip_addesses = vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))];
    //    let mut mn = MobileNetworkCore::new(vec![ran], vec![usr], ip_addesses);

    //    let event_subscriber = EventSubscriber::new(
    //        Url::parse("http://test:8080").unwrap(),
    //        PdnConnectionEvent,
    //        vec![1, 2, 3],
    //    );

    //    //execute
    //    mn.add_subscriber(event_subscriber);

    //    //verify
    //    assert_eq!(mn.event_subscribers.len(), 1);

    //    //execute
    //    let subscribers = mn.get_subscribers();
    //    assert_eq!(subscribers.len(), 1);
    //}

    //#[test]
    //fn get_events() {
    //    //setup
    //    let position = Point::new(0.5, 0.5);
    //    let ran = Ran::new(1, position, 0.5);
    //    let usr = User::new(0, position, 1.0, &(-50.0..50.));
    //    let ip_addesses = vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))];
    //    let mut mn = MobileNetworkCore::new(vec![ran], vec![usr], ip_addesses);

    //    //execute
    //    mn.try_connect_orphans();
    //    mn.generate_location_events();

    //    //verify
    //    assert_eq!(mn.events.len(), 3);

    //    //execute
    //    let events = mn.get_events();
    //    assert_eq!(events.len(), 3);
    //}
}
