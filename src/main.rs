mod application;
mod application_runtime;
mod edge_data_center;
mod mobile_network_core;
mod mobile_network_core_endpoints;
mod mobile_network_core_event;
mod network;
mod network_endpoints;
mod pdu_session;
mod ran;
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
    get_connected_users, get_events, get_rans, get_subscribers, get_users, post_subscribers,
    update_user_positions, MobileNetworkCoreWrapper,
};
use network::Network;
use network_endpoints::{
    add_application, delete_application, get_applications, get_edge_data_centers, NetworkWrapper,
};
use ran::Ran;
use rand::prelude::*;
use simple_logger::SimpleLogger;
use user::User;

fn random_point(rng: &mut ThreadRng, range: &Range<f64>) -> Point {
    let x: f64 = rng.gen_range(range.start..range.end);
    let y: f64 = rng.gen_range(range.start..range.end);
    Point::new(x, y)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();
    let range = -500.0..500.;
    let num_users = 32;
    let user_velocdity = 1.5;
    let num_rans = 16;
    let num_edge_data_centers = 8;
    let num_applications = 8;

    let mut rng = rand::thread_rng();

    let users = (0u32..)
        .take(num_users)
        .map(|id| (id, random_point(&mut rng, &range)))
        .map(|(id, starting_point)| User::new(id, starting_point, user_velocdity, &range))
        .collect();

    let rans = (0u32..)
        .take(num_rans)
        .map(|id| (id, random_point(&mut rng, &range)))
        .map(|(id, point)| Ran::new(id, point, 100.0))
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

    let mut edge_data_centers: Vec<EdgeDataCenter> = (0u32..)
        .take(num_edge_data_centers)
        .map(|id| (id, random_point(&mut rng, &range)))
        .map(|(id, starting_point)| {
            EdgeDataCenter::new(id, &format!("edc: {}", id), starting_point)
        })
        .collect();

    for id in 0..num_applications {
        edge_data_centers[0].add_application(id).unwrap();
    }

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
                    .service(delete_application),
            )
            .service(
                web::scope("/mobile_network")
                    .service(get_users)
                    .service(get_connected_users)
                    .service(get_rans)
                    .service(get_events)
                    .service(get_subscribers)
                    .service(post_subscribers)
                    .service(update_user_positions),
            )
            .app_data(network_wrapper_data.clone())
            .app_data(mnc_wrapper_data.clone())
            .wrap(cors)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}
