use actix_web::{
    get, post,
    web::{Data, Json},
    Responder,
};
use mongodb::Database;
use tokio::sync::RwLock;

use crate::{
    mobile_network_core::MobileNetworkCore, network_endpoints::NetworkWrapper,
    pdu_session::PDUSession, ran::Ran, user::User,
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
    network_wrapper: Data<NetworkWrapper>,
    database: Data<Database>,
) -> impl Responder {
    let mut network = network_wrapper.network.write().await;
    let mut mnc = mobile_network_core_wrapper
        .mobile_network_core
        .write()
        .await;
    mnc.try_connect_orphans(&database).await;
    mnc.update_user_positions(&database).await;
    mnc.generate_location_events(&database).await;
    mnc.use_applications(&mut network, &database).await;
    "OK"
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
