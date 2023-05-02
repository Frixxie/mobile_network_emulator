use std::net::{Ipv4Addr, Ipv6Addr};

use url::Url;

use geo::{Point, Polygon};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EventKind {
    PdnConnectionEvent,
    LocationReporting,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Event {
    PdnConnectionEvent(PdnConnectionInformation),
    LocationReporting(LocationInfo),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocationInfo {
    age_of_location_info: u32,
    cell_id: String,
    e_node_b_id: String,
    routing_area_id: String,
    tracking_area_id: String,
    plmn_id: String,
    twan_id: String,
    geographic_area: GeographicArea,
    civic_address: CivicAddress,
    position_method: Vec<PositioningMethod>,
    qos_fulfill_ind: AccuracyFulfillmentIndicator,
    ue_velocity: VelocityEstimate,
    ldr_type: LdrType,
    achieved_qos: MinorLocationQoS,
}

impl LocationInfo {
    pub fn new(
        age_of_location_info: u32,
        cell_id: String,
        e_node_b_id: String,
        routing_area_id: String,
        tracking_area_id: String,
        plmn_id: String,
        twan_id: String,
        geographic_area: GeographicArea,
        civic_address: CivicAddress,
        position_method: Vec<PositioningMethod>,
        qos_fulfill_ind: AccuracyFulfillmentIndicator,
        ue_velocity: VelocityEstimate,
        ldr_type: LdrType,
        achieved_qos: MinorLocationQoS,
    ) -> Self {
        Self {
            age_of_location_info,
            cell_id,
            e_node_b_id,
            routing_area_id,
            tracking_area_id,
            plmn_id,
            twan_id,
            geographic_area,
            civic_address,
            position_method,
            qos_fulfill_ind,
            ue_velocity,
            ldr_type,
            achieved_qos,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GeographicArea {
    Point(Point),
    PointUncertainCircle,
    PointUncertaintyEllipse,
    Polygon(Polygon),
    PointAltitude,
    PointAlititudeUncertainity,
    EllipsoidArc,
}

//We do not care about this struct it should just be there i guess
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CivicAddress;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PositioningMethod {
    Cellid,
    Ecid,
    Otdoa,
    BarometricPressure,
    Wlan,
    Bluetooth,
    Mbs,
    MotionSensor,
    DlTdoa,
    DlAod,
    MultiRtt,
    NrEcid,
    UlTdoa,
    UlAoa,
    NetworkSpecific,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AccuracyFulfillmentIndicator {
    RequestedAccuracyFulfilled,
    RequestedAccuracyNotFulfilled,
}

//Based on spec this is way more complicated
pub type VelocityEstimate = f64;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LdrType {
    UeAvailable,
    Periodic,
    EnteringIntoArea,
    LeavingFromArea,
    BeingInsideArea,
    Motion,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MinorLocationQoS {
    h_accuracy: f64,
    v_accuracy: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PdnType {
    Ipv4,
    Ipv6,
    Ipv4v6,
    NonIP,
    Ethernet,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PdnConnectionStatus {
    CREATED,
    RELEASED,
}

impl PdnConnectionStatus {
    pub fn created() -> PdnConnectionStatus {
        Self::CREATED
    }
    pub fn released() -> PdnConnectionStatus {
        Self::RELEASED
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InterfaceIndication {
    ExposureFunction,
    PdnGateway,
}

impl InterfaceIndication {
    pub fn exposure_function() -> InterfaceIndication {
        Self::ExposureFunction
    }
    pub fn released() -> InterfaceIndication {
        Self::PdnGateway
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MacAddr {
    mac_addr48: String,
}

impl Default for MacAddr {
    fn default() -> Self {
        Self {
            mac_addr48: "not interesting for now".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PdnConnectionInformation {
    status: PdnConnectionStatus,
    apn: String,
    pdn_type: PdnType,
    interface_ind: InterfaceIndication,
    ipv4_addr: Ipv4Addr,
    ipv6_addrs: Option<Vec<Ipv6Addr>>,
    mac_addrs: Option<Vec<MacAddr>>,
}

impl PdnConnectionInformation {
    pub fn new(
        status: PdnConnectionStatus,
        pdn_type: PdnType,
        interface_ind: InterfaceIndication,
        ipv4_addr: Ipv4Addr,
    ) -> Self {
        Self {
            status,
            apn: "Default".to_string(),
            pdn_type,
            interface_ind,
            ipv4_addr,
            ipv6_addrs: None,
            mac_addrs: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MobileNetworkCoreEvent {
    event: Event,
}

impl MobileNetworkCoreEvent {
    pub fn new(event: Event) -> Self {
        Self { event }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventSubscriber {
    notify_endpoint: String,
    kind: EventKind,
    user_ids: Vec<u32>,
}

impl EventSubscriber {
    pub fn new(notify_endpoint: Url, kind: EventKind, user_ids: Vec<u32>) -> Self {
        EventSubscriber {
            notify_endpoint: notify_endpoint.as_str().to_string(),
            kind,
            user_ids,
        }
    }

    pub fn get_event_type(&self) -> &EventKind {
        &self.kind
    }

    pub fn get_notify_endpoint(&self) -> Url {
        Url::parse(&self.notify_endpoint).unwrap()
    }

    pub fn get_user_ids(&self) -> Vec<&u32> {
        self.user_ids.iter().collect()
    }
}
