use url::Url;

use crate::mobule_network_core_event::EventType;

pub struct Subscription {
    url: Url,
    kind: EventType,
}

impl Subscription {
    pub fn new(url: Url, kind: EventType) -> Self {
        Subscription { url, kind }
    }
}
