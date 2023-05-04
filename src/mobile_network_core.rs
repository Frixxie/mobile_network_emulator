use std::{
    collections::HashSet,
    net::{IpAddr, Ipv4Addr},
    time::{SystemTime, UNIX_EPOCH},
};

use geo::{Contains, Point};
use log::info;
use rand::seq::SliceRandom;
use reqwest::Client;
use serde::Serialize;

use crate::{
    application::Application,
    mobile_network_core_event::{
        AccuracyFulfillmentIndicator, CivicAddress, Event,
        EventKind::{LocationReporting, PdnConnectionEvent},
        EventSubscriber, GeographicArea, InterfaceIndication, LdrType, LocationInfo,
        MinorLocationQoS, MobileNetworkCoreEvent, PdnConnectionInformation, PdnConnectionStatus,
        PdnType, PositioningMethod,
    },
    network::Network,
    pdu_session::PDUSession,
    ran::Ran,
    user::User,
};

#[derive(Clone, Debug, Serialize)]
pub struct Subscriber {
    subscriber: EventSubscriber,
    recieved_events: HashSet<MobileNetworkCoreEvent>,
}

impl Subscriber {
    pub fn new(subscriber: EventSubscriber) -> Self {
        Self {
            subscriber,
            recieved_events: HashSet::new(),
        }
    }

    pub fn get_subscriber(&self) -> &EventSubscriber {
        &self.subscriber
    }
}

pub struct MobileNetworkCore {
    rans: Vec<Ran>,
    orphans: Vec<User>,
    available_ip_addresses: Vec<IpAddr>,
    events: Vec<MobileNetworkCoreEvent>,
    event_subscribers: Vec<Subscriber>,
    http_client: Client,
}

impl MobileNetworkCore {
    pub fn new(rans: Vec<Ran>, orphans: Vec<User>, ip_addesses: Vec<IpAddr>) -> Self {
        MobileNetworkCore {
            rans,
            orphans,
            available_ip_addresses: ip_addesses,
            events: Vec::new(),
            event_subscribers: Vec::new(),
            http_client: Client::new(),
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
                    let pdu_session = PDUSession::new(tmp_orphans.pop().unwrap(), ip_address, &ran);
                    self.events.push(Self::create_location_reporting_event(
                        &ran.get_id().to_string(),
                        pdu_session.user().current_pos(),
                        LdrType::EnteringIntoArea,
                        pdu_session.user().get_id(),
                    ));
                    let v4addr = match ip_address {
                        IpAddr::V4(v4addr) => v4addr,
                        _ => unreachable!(),
                    };
                    self.events.push(Self::create_pdn_connection_event(
                        v4addr,
                        pdu_session.user().get_id(),
                    ));
                    ran.connect_user(pdu_session);
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
                    user.get_id(),
                ));
                self.events
                    .push(Self::release_pdn_connection_event(v4addr, user.get_id()));
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
        for pdu_session in connected_users.iter() {
            self.events.push(Self::create_location_reporting_event(
                &pdu_session.get_ran().get_id().to_string(),
                pdu_session.user().current_pos(),
                LdrType::Motion,
                pdu_session.user().get_id(),
            ));
        }
    }

    pub async fn use_some_applications(&self, network: &mut Network) {
        let connected_users = self.get_connected_users();
        let some_users =
            connected_users.choose_multiple(&mut rand::thread_rng(), connected_users.len() / 2);
        let applications: Vec<Application> =
            network.get_applictions().into_iter().cloned().collect();

        for user in some_users {
            let application = match applications.choose(&mut rand::thread_rng()) {
                Some(application) => application,
                None => {
                    info!("The application list is empty there is no application accessed for user with ip: {}", user.ip());
                    break;
                }
            };
            info!("User with id {} and ip {} is using application {}", user.user().get_id(), user.ip(), application.id());
            network
                .use_application(user, &application, &user.get_ran().get_position())
                .await
                .unwrap();
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

    pub fn add_subscriber(&mut self, event_subscriber: EventSubscriber) {
        self.event_subscribers
            .push(Subscriber::new(event_subscriber));
    }

    pub fn get_subscribers(&self) -> Vec<&Subscriber> {
        self.event_subscribers.iter().collect()
    }

    pub async fn publish_events(&mut self) {
        for subscriber in self.event_subscribers.iter_mut() {
            for event in self
                .events
                .iter()
                .filter(|event| event.get_event_type() == subscriber.subscriber.get_event_type())
            {
                if !subscriber.recieved_events.contains(event)
                    && subscriber
                        .subscriber
                        .get_user_ids()
                        .contains(&&event.get_user_id())
                {
                    self.http_client
                        .post(subscriber.subscriber.get_notify_endpoint())
                        .json::<MobileNetworkCoreEvent>(&event)
                        .send()
                        .await
                        .unwrap();
                    subscriber.recieved_events.insert(event.clone());
                }
            }
        }
    }

    pub fn get_events(&self) -> Vec<&MobileNetworkCoreEvent> {
        self.events.iter().collect()
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

    use std::net::Ipv4Addr;

    use super::*;
    use geo::Point;
    use url::Url;

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

    #[test]
    fn generate_location_events() {
        //setup
        let position = Point::new(0.5, 0.5);
        let ran = Ran::new(1, position, 0.5);
        let usr = User::new(0, position, 1.0, &(-50.0..50.));
        let ip_addesses = vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))];
        let mut mn = MobileNetworkCore::new(vec![ran], vec![usr], ip_addesses);

        //execute
        mn.try_connect_orphans();
        mn.generate_location_events();

        //verify
        assert_eq!(mn.events.len(), 3);
    }

    #[test]
    fn get_rans() {
        //setup
        let position = Point::new(0.5, 0.5);
        let ran = Ran::new(1, position, 0.5);
        let usr = User::new(0, position, 1.0, &(-50.0..50.));
        let ip_addesses = vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))];
        let mn = MobileNetworkCore::new(vec![ran], vec![usr], ip_addesses);

        //execute
        let rans = mn.get_rans();

        //verify
        assert_eq!(rans.len(), 1);
    }

    #[test]
    fn get_users() {
        //setup
        let position = Point::new(0.5, 0.5);
        let ran = Ran::new(1, position, 0.5);
        let usr = User::new(0, position, 1.0, &(-50.0..50.));
        let ip_addesses = vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))];
        let mn = MobileNetworkCore::new(vec![ran], vec![usr], ip_addesses);

        //execute
        let users = mn.get_all_users();

        //verify
        assert_eq!(users.len(), 1);
    }

    #[test]
    fn get_connected_users() {
        //setup
        let position = Point::new(0.5, 0.5);
        let ran = Ran::new(1, position, 0.5);
        let usr = User::new(0, position, 1.0, &(-50.0..50.));
        let ip_addesses = vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))];
        let mut mn = MobileNetworkCore::new(vec![ran], vec![usr], ip_addesses);
        mn.try_connect_orphans();

        //execute
        let users = mn.get_connected_users();

        //verify
        assert_eq!(users.len(), 1);
    }

    #[test]
    fn add_subscriber() {
        //setup
        let position = Point::new(0.5, 0.5);
        let ran = Ran::new(1, position, 0.5);
        let usr = User::new(0, position, 1.0, &(-50.0..50.));
        let ip_addesses = vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))];
        let mut mn = MobileNetworkCore::new(vec![ran], vec![usr], ip_addesses);

        let event_subscriber = EventSubscriber::new(
            Url::parse("http://test:8080").unwrap(),
            PdnConnectionEvent,
            vec![1, 2, 3],
        );

        //execute
        mn.add_subscriber(event_subscriber);

        //verify
        assert_eq!(mn.event_subscribers.len(), 1);
    }

    #[test]
    fn get_subscribers() {
        //setup
        let position = Point::new(0.5, 0.5);
        let ran = Ran::new(1, position, 0.5);
        let usr = User::new(0, position, 1.0, &(-50.0..50.));
        let ip_addesses = vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))];
        let mut mn = MobileNetworkCore::new(vec![ran], vec![usr], ip_addesses);

        let event_subscriber = EventSubscriber::new(
            Url::parse("http://test:8080").unwrap(),
            PdnConnectionEvent,
            vec![1, 2, 3],
        );

        //execute
        mn.add_subscriber(event_subscriber);

        //verify
        assert_eq!(mn.event_subscribers.len(), 1);

        //execute
        let subscribers = mn.get_subscribers();
        assert_eq!(subscribers.len(), 1);
    }

    #[test]
    fn get_events() {
        //setup
        let position = Point::new(0.5, 0.5);
        let ran = Ran::new(1, position, 0.5);
        let usr = User::new(0, position, 1.0, &(-50.0..50.));
        let ip_addesses = vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))];
        let mut mn = MobileNetworkCore::new(vec![ran], vec![usr], ip_addesses);

        //execute
        mn.try_connect_orphans();
        mn.generate_location_events();

        //verify
        assert_eq!(mn.events.len(), 3);

        //execute
        let events = mn.get_events();
        assert_eq!(events.len(), 3);
    }
}
