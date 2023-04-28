use std::net::{Ipv4Addr, Ipv6Addr};

use geo::{Point, Polygon};

pub enum EventType {
    PdnConnectionEvent(PdnConnectionInformation),
    LocationReporting(LocationInfo),
}

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
pub struct CivicAddress;

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

pub enum AccuracyFulfillmentIndicator {
    RequestedAccuracyFulfilled,
    RequestedAccuracyNotFulfilled,
}

//Based on spec this is way more complicated
pub type VelocityEstimate = f64;

pub enum LdrType {
    UeAvailable,
    Periodic,
    EnteringIntoArea,
    LeavingFromArea,
    BeingInsideArea,
    Motion,
}

pub struct MinorLocationQoS {
    h_accuracy: f64,
    v_accuracy: f64,
}

pub enum PdnType {
    Ipv4,
    Ipv6,
    Ipv4v6,
    NonIP,
    Ethernet,
}

pub enum PdnConnectionStatus {
    CREATED,
    RELEASED,
}

pub enum InterfaceIndication {
    ExposureFunction,
    PdnGateway,
}

pub struct MacAddr {
    mac_addr48: String,
}

pub struct PdnConnectionInformation {
    status: PdnConnectionStatus,
    apn: String,
    pdn_type: PdnType,
    interface_ind: InterfaceIndication,
    ipv4_addr: Ipv4Addr,
    ipv6_addrs: Vec<Ipv6Addr>,
    mac_addrs: Vec<MacAddr>,
}

pub struct MobileNetworkCoreEvent {
    kind: EventType,
    description: String,
}

impl MobileNetworkCoreEvent {
    pub fn new(kind: EventType, description: &str) -> Self {
        MobileNetworkCoreEvent {
            kind,
            description: description.to_string(),
        }
    }
}
