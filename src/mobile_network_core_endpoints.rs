use actix_web::{
    get, post,
    web::{Data, Json},
    Responder,
};
use tokio::sync::RwLock;

use crate::{
    mobile_network_core::MobileNetworkCore,
    mobile_network_core_event::{EventSubscriber, MobileNetworkCoreEvent},
    pdu_session::PDUSession,
    ran::Ran,
    user::User,
};

pub struct MobileNetworkCoreWrapper {
    mobile_network_core: RwLock<MobileNetworkCore>,
}

impl MobileNetworkCoreWrapper {
    pub fn new(mobile_network_core: MobileNetworkCore) -> Self {
        MobileNetworkCoreWrapper {
            mobile_network_core: RwLock::new(mobile_network_core),
        }
    }
}

#[get("/users")]
pub async fn get_users(
    mobile_network_core_wrapper: Data<MobileNetworkCoreWrapper>,
) -> impl Responder {
    let users: Vec<User> = mobile_network_core_wrapper
        .mobile_network_core
        .read()
        .await
        .get_all_users()
        .into_iter()
        .cloned()
        .collect();
    Json(users)
}

#[get("/connected_users")]
pub async fn get_connected_users(
    mobile_network_core_wrapper: Data<MobileNetworkCoreWrapper>,
) -> impl Responder {
    let users: Vec<PDUSession> = mobile_network_core_wrapper
        .mobile_network_core
        .read()
        .await
        .get_connected_users()
        .into_iter()
        .cloned()
        .collect();
    Json(users)
}

#[post("/update_user_positions")]
pub async fn update_user_positions(
    mobile_network_core_wrapper: Data<MobileNetworkCoreWrapper>,
) -> impl Responder {
    let mut mnc = mobile_network_core_wrapper
        .mobile_network_core
        .write()
        .await;
    mnc.try_connect_orphans();
    mnc.update_user_positions();
    "OK"
}

/// This function makes a subscriber subscribe to events
#[post("/subscribers")]
pub async fn post_subscribers(
    mobile_network_core_wrapper: Data<MobileNetworkCoreWrapper>,
    event_subscription: Json<EventSubscriber>,
) -> impl Responder {
    let mut mnc = mobile_network_core_wrapper
        .mobile_network_core
        .write()
        .await;
    mnc.add_subscriber(event_subscription.into_inner());
    "OK"
}

#[get("/subscribers")]
pub async fn get_subscribers(
    mobile_network_core_wrapper: Data<MobileNetworkCoreWrapper>,
) -> impl Responder {
    let subscribers: Vec<EventSubscriber> = mobile_network_core_wrapper
        .mobile_network_core
        .read()
        .await
        .get_subscribers()
        .into_iter()
        .cloned()
        .collect();
    Json(subscribers)
}

#[get("/events")]
pub async fn get_events(
    mobile_network_core_wrapper: Data<MobileNetworkCoreWrapper>,
) -> impl Responder {
    let events: Vec<MobileNetworkCoreEvent> = mobile_network_core_wrapper
        .mobile_network_core
        .read()
        .await
        .get_events()
        .into_iter()
        .cloned()
        .collect();
    Json(events)
}

#[get("/rans")]
pub async fn get_rans(
    mobile_network_core_wrapper: Data<MobileNetworkCoreWrapper>,
) -> impl Responder {
    let users: Vec<Ran> = mobile_network_core_wrapper
        .mobile_network_core
        .read()
        .await
        .get_rans()
        .into_iter()
        .cloned()
        .collect();
    Json(users)
}
