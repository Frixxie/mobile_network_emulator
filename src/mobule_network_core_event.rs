pub enum EventType {
    MONITORING_EVENT,
    PDU_CONNECTION_EVENT,
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
