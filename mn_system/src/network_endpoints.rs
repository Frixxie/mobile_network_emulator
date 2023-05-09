use actix_web::error::ErrorInternalServerError;
use actix_web::web::{Data, Json, Path};
use actix_web::{delete, get, post, Responder};
use tokio::sync::RwLock;

use crate::application::Application;

use crate::edge_data_center::EdgeDataCenter;
use crate::network::Network;

pub struct NetworkWrapper {
    pub network: RwLock<Network>,
}

impl NetworkWrapper {
    pub fn new(network: Network) -> Self {
        NetworkWrapper {
            network: RwLock::new(network),
        }
    }
}

#[get("/edge_data_centers")]
pub async fn get_edge_data_centers(network_wrapper: Data<NetworkWrapper>) -> impl Responder {
    let edge_data_centers: Vec<EdgeDataCenter> = network_wrapper
        .network
        .read()
        .await
        .get_edge_data_centers()
        .into_iter()
        .cloned()
        .collect();
    Json(edge_data_centers)
}

#[get("/edge_data_centers/{id}/applications")]
pub async fn get_applications(
    id: Path<u32>,
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
        .cloned()
        .collect();
    Json(applications)
}

#[post("/edge_data_centers/{edc_id}/applications/{application_id}")]
pub async fn add_application(
    edc_id: Path<u32>,
    application_id: Path<u32>,
    network_wrapper: Data<NetworkWrapper>,
) -> Result<impl Responder, actix_web::Error> {
    match network_wrapper
        .network
        .write()
        .await
        .get_mut_edge_data_center(*edc_id)
        .unwrap()
        .add_application(*application_id)
    {
        Ok(url) => Ok(url.to_string()),
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}

#[delete("/edge_data_centers/{edc_id}/applications/{application_id}")]
pub async fn delete_application(
    edc_id: Path<u32>,
    application_id: Path<u32>,
    network_wrapper: Data<NetworkWrapper>,
) -> Result<impl Responder, actix_web::Error> {
    match network_wrapper
        .network
        .write()
        .await
        .get_mut_edge_data_center(*edc_id)
        .unwrap()
        .remove_application(*application_id)
    {
        Ok(_) => Ok("OK"),
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}

#[get("/edge_data_centers/{edc_id}/applications/{application_id}/total_usages")]
pub async fn get_total_application_usage(
    edc_id: Path<u32>,
    application_id: Path<u32>,
    network_wrapper: Data<NetworkWrapper>,
) -> impl Responder {
    Json(
        network_wrapper
            .network
            .read()
            .await
            .get_total_application_usage(*edc_id, *application_id)
            .unwrap(),
    )
}

#[get("/edge_data_centers/{edc_id}/applications/{application_id}/get_usage")]
pub async fn get_application_usage(
    edc_id: Path<u32>,
    application_id: Path<u32>,
    network_wrapper: Data<NetworkWrapper>,
) -> impl Responder {
    Json(
        network_wrapper
            .network
            .read()
            .await
            .get_application_usage(*edc_id, *application_id)
            .unwrap(),
    )
}
