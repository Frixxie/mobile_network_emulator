use std::{fs::File, io::BufReader};

use geo::{coord, LineString, Polygon, Rect};
use geocoding::{Forward, Openstreetmap, Point};
use gpx::Gpx;
mod mobile_network;
mod ran;
mod user;

// fn create_polygon_from_point(point: Point) -> Polygon {
//     let p1 = point.clone();
//     let mut p2 = point.clone();
//     p2.set_x(p1.x() + 1.0).set_y(p1.y() + 1.0);
//     let mut p3 = point.clone();
//     p3.set_x(p1.x() + 1.0);
//     let inner_thingy = vec![p1, p2, p1, p3];
//     Polygon::new(LineString::from(inner_thingy), vec![])
// }

fn main() {
    // let file = File::open("6938758.gpx").unwrap();
    // // let file = File::open("wikipedia_example.gpx").unwrap();
    // let reader = BufReader::new(file);

    // let gpx: Gpx = gpx::read(reader).unwrap();
    // // println!("{:?}", gpx);

    // for track in gpx.tracks.iter() {
    //     for segment in track.segments.iter() {
    //         for point in segment.points.iter() {
    //             println!("{:?}", point);
    //         }
    //     }
    // }
    //

    // let osm = Openstreetmap::new();
    // let address = "Tromsø, Troms";
    // let res: Vec<Point> = osm.forward(&address).unwrap();
    // println!("{:?}", res);

    let mut mobile_network = mobile_network::MobileNetwork::new();
    let rect = Rect::new(coord! { x: 0., y: 0.}, coord! { x: 1., y: 1.});
    let gnb_tromso = ran::GNodeB::new(1, (0.5, 0.5).into(), rect);
    let mut ue = user::User::new(0);
    let ue_point = Point::new(0.5, 0.5);
    ue.add_trail(vec![ue_point]);

    // println!("{}", ue);

    mobile_network.add_g_node_bs(&mut vec![gnb_tromso]);
    mobile_network.add_ues(&mut vec![ue]);

    let ues_in_range = mobile_network.get_ues_in_reach();
    for ue in ues_in_range {
        println!("{}", ue);
    }
}
