mod application;
mod application_runtime;
mod edge_data_center;
mod mobile_network_core;
mod mobile_network_core_endpoints;
mod mobule_network_core_event;
mod network;
mod network_endpoints;
mod pdu_session;
mod ran;
mod subscription;
mod user;
use std::{
    iter::repeat_with,
    net::{IpAddr, Ipv4Addr},
    ops::Range,
};

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use edge_data_center::EdgeDataCenter;
use geo::Point;
use mobile_network_core::MobileNetworkCore;
use mobile_network_core_endpoints::{
    get_connected_users, get_rans, get_users, update_user_positions, MobileNetworkCoreWrapper,
};
use network::Network;
use network_endpoints::{
    add_application, delete_application, get_applications, get_edge_data_centers, NetworkWrapper,
};
use ran::Ran;
use rand::prelude::*;
use user::User;

fn random_point(rng: &mut ThreadRng, range: &Range<f64>) -> Point {
    let x: f64 = rng.gen_range(range.start..range.end);
    let y: f64 = rng.gen_range(range.start..range.end);
    Point::new(x, y)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let range = -500.0..500.;
    let num_users = 5;
    let num_rans = 16;
    let num_edge_data_centers = 16;

    let mut rng = rand::thread_rng();

    let users = (0u32..)
        .take(num_users)
        .map(|id| (id, random_point(&mut rng, &range)))
        .map(|(id, starting_point)| {
            let mut user = User::new(id);
            let path = User::generate_user_path(&range, starting_point, 1024);
            user.add_path(path);
            user
        })
        .collect();

    let rans = repeat_with(|| random_point(&mut rng, &range))
        .take(num_rans)
        .map(|point| Ran::new(point, 150.0))
        .collect();

    let ip_addresses = repeat_with(|| (rng.gen(), rng.gen(), rng.gen(), rng.gen()))
        .take(num_users)
        .map(|(first, second, thrid, foruth)| {
            IpAddr::V4(Ipv4Addr::new(first, second, thrid, foruth))
        })
        .collect();

    let mnc = MobileNetworkCore::new(rans, users, ip_addresses);
    let mnc_wrapper = MobileNetworkCoreWrapper::new(mnc);
    let mnc_wrapper_data = Data::new(mnc_wrapper);

    let edge_data_centers: Vec<EdgeDataCenter> = (0u32..)
        .take(num_edge_data_centers)
        .map(|id| (id, random_point(&mut rng, &range)))
        .map(|(id, starting_point)| {
            EdgeDataCenter::new(id, &format!("edc: {}", id), starting_point)
        })
        .collect();

    let network = Network::new(edge_data_centers);
    let network_wrapper = NetworkWrapper::new(network);
    let network_wrapper_data = Data::new(network_wrapper);

    HttpServer::new(move || {
        let cors = actix_cors::Cors::permissive();
        App::new()
            .service(
                web::scope("/network")
                    .service(get_edge_data_centers)
                    .service(get_applications)
                    .service(add_application)
                    .service(delete_application)
                    .app_data(network_wrapper_data.clone()),
            )
            .service(
                web::scope("/mobile_network")
                    .service(get_users)
                    .service(get_connected_users)
                    .service(get_rans)
                    .service(update_user_positions)
                    .app_data(mnc_wrapper_data.clone()),
            )
            .wrap(cors)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}
