use geo::{coord, Rect};
use geocoding::Point;
mod mobile_network;
mod ran;
mod user;

fn main() {
    let mut mobile_network = mobile_network::MobileNetwork::new();
    let rect = Rect::new(coord! { x: 0., y: 0.}, coord! { x: 1., y: 1.});
    let gnb_tromso = ran::GNodeB::new(1, (0.5, 0.5).into(), rect);
    let mut ue = user::User::new(0);
    let ue_point = Point::new(0.5, 0.5);
    ue.add_trail(vec![ue_point]);

    mobile_network.add_g_node_bs(&mut vec![gnb_tromso]);
    mobile_network.add_ues(&mut vec![ue]);

    let ues_in_range = mobile_network.get_ues_in_reach();
    for ue in ues_in_range {
        println!("{}", ue);
    }
}
