mod application;
mod application_runtime;
mod edge_data_center;
mod mobile_network_core;
mod mobule_network_core_event;
mod network;
mod pdu_session;
mod ran;
mod subscription;
mod user;
use std::ops::Range;

use geo::Point;
use rand::prelude::*;
use user::User;

fn random_point(rng: &mut ThreadRng, range: &Range<f64>) -> Point {
    let min = range.start;
    let max = range.start;
    let x: f64 = rng.gen_range(min..max);
    let y: f64 = rng.gen_range(min..max);
    Point::new(x, y)
}

fn main() {
    let range = -1000.0..1000.0;
    let mut rng = rand::thread_rng();
    let mut users: Vec<User> = (0u32..)
        .map(|id| (id, random_point(&mut rng, &range)))
        .map(|(id, starting_point)| {
            let mut user = User::new(id);
            let path = User::generate_user_path(&range, starting_point, 1024);
            user.add_path(path);
            user
        })
        .collect();
}
