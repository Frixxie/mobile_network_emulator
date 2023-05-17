mod application;
mod application_runtime;
mod edge_data_center;
mod mobile_network_core;
mod mobile_network_core_endpoints;
mod mobile_network_exposure;
mod mobile_network_exposure_endpoints;
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
    get_connected_users, get_rans, get_users, update_user_positions, MobileNetworkCoreWrapper,
};
use mobile_network_exposure::MobileNetworkExposure;
use mobile_network_exposure_endpoints::{
    get_events, get_subscribers, post_subscribers, publish_events, MobileNetworkExposureWrapper,
};
use network::Network;
use network_endpoints::{
    add_application, delete_application, get_applications, get_edge_data_centers,
    get_total_application_usage, NetworkWrapper,
};
use poisson_diskus::bridson;
use ran::Ran;
use rand::prelude::*;
use simple_logger::SimpleLogger;
use structopt::StructOpt;
use user::User;

fn random_point(rng: &mut ThreadRng, range: &Range<f64>) -> Point {
    let x: f64 = rng.gen_range(range.start..range.end);
    let y: f64 = rng.gen_range(range.start..range.end);
    Point::new(x, y)
}

fn poisson_points(range: &Range<f64>, rmin: f32) -> Vec<Point> {
    let input_range = [range.start, range.end];
    bridson(&input_range, rmin.into(), 30, true)
        .unwrap()
        .into_iter()
        .map(|a| Point::new(a[0], a[1]))
        .collect()
}

#[derive(Debug, StructOpt)]
#[structopt(name = "mn_system", about = "Backend for mobile_network_system")]
struct Opt {
    #[structopt(short, long, default_value = "0.0.0.0")]
    host: String,

    #[structopt(short, long, default_value = "8080")]
    port: u16,

    #[structopt(short, long, default_value = "mongodb://localhost:27017/")]
    db_connection_string: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();
    let opts = Opt::from_args();
    let client_options = mongodb::options::ClientOptions::parse(&opts.db_connection_string)
        .await
        .unwrap();
    let db_client = mongodb::Client::with_options(client_options).unwrap();
    let database = db_client.database("mn_system");

    let db_client_data = Data::new(database);

    let bounds = -500.0..500.;
    let num_users = 32;
    let user_velocdity = 1.5;
    let _num_rans = 16;
    let num_edge_data_centers = 8;
    let num_applications = 8;

    let mut rng = rand::thread_rng();

    let users = (0u32..)
        .take(num_users)
        .map(|id| (id, random_point(&mut rng, &bounds)))
        .map(|(id, starting_point)| User::new(id, starting_point, user_velocdity, &bounds))
        .collect();

    // let rans = (0u32..)
    //     .take(num_rans)
    //     .map(|id| (id, random_point(&mut rng, &bounds)))
    //     .map(|(id, point)| Ran::new(id, point, 100.0))
    //     .collect();
    let rans = poisson_points(&(1000.0..1000.0), 150.0)
        .into_iter()
        .enumerate()
        .map(|(id, point)| {
            let p = Point::new(point.x() - 500.0, point.y() - 500.0);
            Ran::new(id as u32, p, 100.0)
        })
        .collect();

    //TODO: Make sure that we only get unique ip addresses
    let ip_addresses = repeat_with(|| (rng.gen(), rng.gen(), rng.gen(), rng.gen()))
        .take(num_users)
        .map(|(first, second, thrid, foruth)| {
            IpAddr::V4(Ipv4Addr::new(first, second, thrid, foruth))
        })
        .collect();

    let mnc = MobileNetworkCore::new(rans, users, ip_addresses);
    let mnc_wrapper = MobileNetworkCoreWrapper::new(mnc);
    let mnc_wrapper_data = Data::new(mnc_wrapper);

    let mnce = MobileNetworkExposure::new();
    let mnce_wrapper = MobileNetworkExposureWrapper::new(mnce);
    let mnce_wrapper_data = Data::new(mnce_wrapper);

    let mut edge_data_centers: Vec<EdgeDataCenter> = (0u32..)
        .take(num_edge_data_centers)
        .map(|id| (id, random_point(&mut rng, &bounds)))
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
                    .service(get_total_application_usage)
                    .service(add_application)
                    .service(delete_application),
            )
            .service(
                web::scope("/mobile_network")
                    .service(get_users)
                    .service(get_connected_users)
                    .service(get_rans)
                    .service(update_user_positions),
            )
            .service(
                web::scope("/mobile_network_exposure")
                    .service(get_events)
                    .service(get_subscribers)
                    .service(post_subscribers)
                    .service(publish_events),
            )
            .app_data(network_wrapper_data.clone())
            .app_data(mnc_wrapper_data.clone())
            .app_data(mnce_wrapper_data.clone())
            .app_data(db_client_data.clone())
            .wrap(cors)
    })
    .bind((opts.host, opts.port))?
    .run()
    .await?;
    Ok(())
}
