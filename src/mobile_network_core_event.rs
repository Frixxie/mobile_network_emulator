use std::net::{Ipv4Addr, Ipv6Addr};

pub enum EventType {
    MonitoringEvent,
    PdnConnectionEvent(PdnConnectionInformation),
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
