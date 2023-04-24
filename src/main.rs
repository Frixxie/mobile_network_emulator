mod application;
mod application_runtime;
mod edge_data_center;
mod network_endpoints;
mod mobile_network_core;
mod mobule_network_core_event;
mod network;
mod pdu_session;
mod ran;
mod subscription;
mod user;
use std::{
    iter::repeat,
    net::{IpAddr, Ipv4Addr},
    ops::Range,
};

use geo::Point;
use mobile_network_core::MobileNetworkCore;
use ran::Ran;
use rand::prelude::*;
use user::User;

fn random_point(rng: &mut ThreadRng, range: &Range<f64>) -> Point {
    let x: f64 = rng.gen_range(range.start..range.end);
    let y: f64 = rng.gen_range(range.start..range.end);
    Point::new(x, y)
}

fn main() {
    let range = -1000.0..1000.0;
    let mut rng = rand::thread_rng();
    let users = (0u32..)
        .take(1024)
        .map(|id| (id, random_point(&mut rng, &range)))
        .map(|(id, starting_point)| {
            let mut user = User::new(id);
            let path = User::generate_user_path(&range, starting_point, 1024);
            user.add_path(path);
            user
        })
        .collect();

    let rans = repeat(random_point(&mut rng, &range))
        .take(32)
        .map(|point| Ran::new(point, 150.0))
        .collect();

    let ip_addresses = repeat((rng.gen(), rng.gen(), rng.gen(), rng.gen()))
        .take(1024)
        .map(|(first, second, thrid, foruth)| {
            IpAddr::V4(Ipv4Addr::new(first, second, thrid, foruth))
        })
        .collect();

    let mut mnc = MobileNetworkCore::new(rans, users, ip_addresses);

    loop {
        mnc.try_connect_orphans();
        mnc.update_user_positions();
        let usrs = mnc.get_connected_users();
        println!("Current connected users {}", usrs.len());
    }
}
