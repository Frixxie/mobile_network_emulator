use actix_web::{
    get, post,
    web::{Data, Json},
    Responder,
};
use mobile_network_core_event::MobileNetworkCoreEvent;
use mongodb::Database;
use tokio::sync::RwLock;

use crate::mobile_network_exposure::{MobileNetworkExposure, EventSubscriber};

pub struct MobileNetworkExposureWrapper {
    mobile_network_core: RwLock<MobileNetworkExposure>,
}

impl MobileNetworkExposureWrapper {
    pub fn new(mobile_network_exposure: MobileNetworkExposure) -> Self {
        MobileNetworkExposureWrapper {
            mobile_network_core: RwLock::new(mobile_network_exposure),
        }
    }
}

/// This function makes a subscriber subscribe to events
#[post("/subscribers")]
pub async fn post_subscribers(
    mobile_network_core_wrapper: Data<MobileNetworkExposureWrapper>,
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
    mobile_network_core_wrapper: Data<MobileNetworkExposureWrapper>,
) -> impl Responder {
    let subscribers: Vec<EventSubscriber> = mobile_network_core_wrapper
        .mobile_network_core
        .read()
        .await
        .get_subscribers()
        .into_iter()
        .map(|subscriber| subscriber.get_subscriber())
        .cloned()
        .collect();
    Json(subscribers)
}

#[get("/events")]
pub async fn get_events(
    mobile_network_core_wrapper: Data<MobileNetworkExposureWrapper>,
    database: Data<Database>,
) -> impl Responder {
    let events: Vec<MobileNetworkCoreEvent> = mobile_network_core_wrapper
        .mobile_network_core
        .read()
        .await
        .get_events(&database)
        .await;
    Json(events)
}

/// Endpoint to publish events
#[post("/events/publish")]
pub async fn publish_events(
    mobile_network_core_wrapper: Data<MobileNetworkExposureWrapper>,
    database: Data<Database>,
) -> impl Responder {
    mobile_network_core_wrapper
        .mobile_network_core
        .write()
        .await
        .publish_events(&database)
        .await;
    "OK"
}
