use std::{
    hash::{Hash, Hasher},
    net::{Ipv4Addr, Ipv6Addr},
    time::{SystemTime, UNIX_EPOCH, Duration},
};

use geo::{Point, Polygon};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventKind {
    PdnConnectionEvent,
    LocationReporting,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Event {
    PdnConnectionEvent(PdnConnectionInformation),
    LocationReporting(LocationInfo),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LocationInfo {
    pub age_of_location_info: u64,
    pub cell_id: String,
    pub e_node_b_id: String,
    pub routing_area_id: String,
    pub tracking_area_id: String,
    pub plmn_id: String,
    pub twan_id: String,
    pub geographic_area: GeographicArea,
    pub civic_address: CivicAddress,
    pub position_method: Vec<PositioningMethod>,
    pub qos_fulfill_ind: AccuracyFulfillmentIndicator,
    pub ue_velocity: VelocityEstimate,
    pub ldr_type: LdrType,
    pub achieved_qos: MinorLocationQoS,
}

impl Eq for LocationInfo {}

impl Hash for LocationInfo {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.e_node_b_id.hash(state);
        self.age_of_location_info.hash(state);
        self.cell_id.hash(state);
        self.routing_area_id.hash(state);
        self.tracking_area_id.hash(state);
        self.plmn_id.hash(state);
        self.twan_id.hash(state);
        self.geographic_area.hash(state);
        self.civic_address.hash(state);
        self.position_method.hash(state);
        self.qos_fulfill_ind.hash(state);
        self.ue_velocity.to_bits().hash(state);
    }
}

impl LocationInfo {
    pub fn new(
        age_of_location_info: u64,
        e_node_b_id: String,
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
            cell_id: "see e_node_b_id".to_string(),
            e_node_b_id,
            routing_area_id: "see e_node_b_id".to_string(),
            tracking_area_id: "see e_node_b_id".to_string(),
            plmn_id: "1".to_string(),
            twan_id: "1".to_string(),
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum GeographicArea {
    Point(Point),
    PointUncertainCircle,
    PointUncertaintyEllipse,
    Polygon(Polygon),
    PointAltitude,
    PointAlititudeUncertainity,
    EllipsoidArc,
}

impl Eq for GeographicArea {}

impl Hash for GeographicArea {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            GeographicArea::Point(point) => {
                point.x().to_bits().hash(state);
                point.y().to_bits().hash(state);
            }
            GeographicArea::PointUncertainCircle => "PointUncertainCircle".hash(state),
            GeographicArea::PointUncertaintyEllipse => "PointUncertaintyEllipse".hash(state),
            GeographicArea::Polygon(polygon) => {
                for point in polygon.exterior() {
                    point.x.to_bits().hash(state);
                    point.y.to_bits().hash(state);
                }
            }
            GeographicArea::PointAltitude => "PointAltitude".hash(state),
            GeographicArea::PointAlititudeUncertainity => "PointAlititudeUncertainity".hash(state),
            GeographicArea::EllipsoidArc => "EllipsoidArc".hash(state),
        }
    }
}

//We do not care about this struct it should just be there i guess
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CivicAddress;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PositioningMethod {
    CellId,
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccuracyFulfillmentIndicator {
    RequestedAccuracyFulfilled,
    RequestedAccuracyNotFulfilled,
}

//Based on spec this is way more complicated
pub type VelocityEstimate = f64;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LdrType {
    UeAvailable,
    Periodic,
    EnteringIntoArea,
    LeavingFromArea,
    BeingInsideArea,
    Motion,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MinorLocationQoS {
    h_accuracy: f64,
    v_accuracy: f64,
}

impl MinorLocationQoS {
    pub fn new(h_accuracy: f64, v_accuracy: f64) -> Self {
        Self {
            h_accuracy,
            v_accuracy,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PdnType {
    Ipv4,
    Ipv6,
    Ipv4v6,
    NonIP,
    Ethernet,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PdnConnectionStatus {
    Created,
    Released,
}

impl PdnConnectionStatus {
    pub fn created() -> PdnConnectionStatus {
        Self::Created
    }
    pub fn released() -> PdnConnectionStatus {
        Self::Released
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PdnConnectionInformation {
    pub status: PdnConnectionStatus,
    pub apn: String,
    pub pdn_type: PdnType,
    pub interface_ind: InterfaceIndication,
    pub ipv4_addr: Ipv4Addr,
    pub ipv6_addrs: Option<Vec<Ipv6Addr>>,
    pub mac_addrs: Option<Vec<MacAddr>>,
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

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct MobileNetworkCoreEvent {
    event: Event,
    kind: EventKind,
    timestamp: Duration,
    user_id: u32,
}

impl MobileNetworkCoreEvent {
    pub fn new(event: Event, kind: EventKind, user_id: u32) -> Self {
        Self {
            event,
            kind,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
            user_id,
        }
    }

    pub fn get_event(&self) -> &Event {
        &self.event
    }

    pub fn get_event_type(&self) -> &EventKind {
        &self.kind
    }

    pub fn get_user_id(&self) -> u32 {
        self.user_id
    }

    pub fn get_timestamp(&self) -> Duration {
        self.timestamp
    }
}
