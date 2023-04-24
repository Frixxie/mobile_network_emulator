use actix_web::delete;
use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, web, Responder};
use serde::Deserialize;
use tokio::sync::RwLock;

use crate::application::Application;
use crate::network::Network;

pub struct NetworkWrapper {
    network: RwLock<Network>,
}

impl NetworkWrapper {
    pub fn new(network: Network) -> Self {
        NetworkWrapper {
            network: RwLock::new(network),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ApplicationForm {
    url: String,
    id: usize,
}

#[get("/{id}/applications")]
pub async fn get_applications(
    id: Path<usize>,
    network_wrapper: Data<NetworkWrapper>,
) -> impl Responder {
    let applications: Vec<Application> = network_wrapper
        .network
        .read()
        .await
        .get_edge_data_center(*id)
        .unwrap()
        .get_applications()
        .into_iter()
        .map(|application| application.clone())
        .collect();
    Json(applications)
}

// #[post("/{id}/applications")]
// pub async fn add_application(
//     id: Path<usize>,
//     network_wrapper: Data<NetworkWrapper>,
//     application: Json<ApplicationForm>,
// ) -> Result<impl Responder, actix_web::Error> {
//     todo!();
// }

// #[delete("/{id}/applications")]
// pub async fn remove_application(
//     id: Path<usize>,
//     network_wrapper: web::Data<NetworkWrapper>,
//     application: web::Json<ApplicationForm>,
// ) -> Result<impl Responder, actix_web::Error> {
//     todo!();
// }
