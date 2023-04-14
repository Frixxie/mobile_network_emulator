use crate::{
    mobule_network_core_event::MobileNetworkCoreEvent, ran::Ran, subscription::Subscription,
    user::User, pdu_session::PDUSession,
};

pub struct MobileNetworkCore {
    rans: Vec<Ran>,
    orphans: Vec<User>,
    events: Vec<MobileNetworkCoreEvent>,
    event_subscribers: Vec<Subscription>,
}

impl MobileNetworkCore {
    pub fn new(rans: Vec<Ran>, orphans: Vec<User>) -> Self {
        todo!();
    }
    pub fn update_user_positions(&mut self) {
        todo!();
    }
    pub fn get_connected_users(&self) -> Vec<&PDUSession> {
        todo!();
    }
    pub fn get_in_reach_users(&self) -> Vec<&User> {
        todo!();
    }
    pub fn get_out_of_reach_users(&self) -> Vec<&User> {
        todo!();
    }
    pub fn connect_some_users(&mut self) {
        todo!();
    }
    pub fn get_events(&self) -> MobileNetworkCoreEvent {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
