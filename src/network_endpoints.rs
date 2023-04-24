use actix_web::error::ErrorInternalServerError;
use actix_web::web::{Data, Json, Path};
use actix_web::{delete, get, post, Responder};
use serde::Deserialize;
use tokio::sync::RwLock;
use url::Url;

use crate::application::Application;

use crate::edge_data_center::EdgeDataCenter;
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
    pub url: String,
    pub id: u32,
}

impl From<ApplicationForm> for Application {
    fn from(value: ApplicationForm) -> Self {
        let url = match Url::parse(&value.url) {
            Ok(url) => url,
            Err(e) => panic!("{}", e),
        };
        Application::new(url, value.id)
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

#[get("/{id}/applications")]
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

#[post("/{id}/applications")]
pub async fn add_application(
    id: Path<u32>,
    network_wrapper: Data<NetworkWrapper>,
    application: Json<ApplicationForm>,
) -> Result<impl Responder, actix_web::Error> {
    match network_wrapper
        .network
        .write()
        .await
        .get_mut_edge_data_center(*id)
        .unwrap()
        .add_application(&application.into_inner().into())
    {
        Ok(url) => Ok(url.to_string()),
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}

#[delete("/{id}/applications")]
pub async fn delete_application(
    id: Path<u32>,
    network_wrapper: Data<NetworkWrapper>,
    application: Json<ApplicationForm>,
) -> Result<impl Responder, actix_web::Error> {
    match network_wrapper
        .network
        .write()
        .await
        .get_mut_edge_data_center(*id)
        .unwrap()
        .remove_application(&application.into_inner().into())
    {
        Ok(_) => Ok("OK"),
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}
