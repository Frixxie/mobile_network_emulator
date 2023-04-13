use geo::Contains;

use crate::{
    ran::GNodeB,
    user::{User, UserState},
};

pub struct MobileNetwork {
    g_node_bs: Vec<GNodeB>,
    ues: Vec<User>,
}

impl MobileNetwork {
    pub fn new() -> Self {
        MobileNetwork {
            g_node_bs: Vec::new(),
            ues: Vec::new(),
        }
    }

    pub fn add_g_node_bs(&mut self, nodes: &mut Vec<GNodeB>) {
        self.g_node_bs.append(nodes);
    }

    pub fn add_ues(&mut self, ues: &mut Vec<User>) {
        self.ues.append(ues);
    }

    pub fn check_ues(&mut self) {
        for ue in self.ues.iter_mut() {
            ue.state = UserState::OutOfReach;
            for gnb in &self.g_node_bs {
                if gnb.contains(ue) {
                    ue.state = UserState::InReach
                }
            }
        }
    }

    pub fn get_ues_in_reach(&self) -> Vec<User> {
        self.ues
            .iter()
            .filter(|ue| {
                if ue.state == UserState::InReach {
                    true
                } else {
                    false
                }
            })
            .map(|ue| ue.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use geo::{coord, Point, Rect};

    use super::*;

    #[test]
    pub fn check_ues() {
        let mut mobile_network = MobileNetwork::new();
        let mut rect = Rect::new(coord! { x: 0., y: 0.}, coord! { x: 1., y: 1.});
        let gnb1 = GNodeB::new(1, (0.5, 0.5).into(), rect);
        rect = Rect::new(coord! { x: 1., y: 1.}, coord! { x: 2., y: 2.});
        let gnb2 = GNodeB::new(2, (1.2, 1.2).into(), rect);
        let mut ue_in_reach = User::new(0);
        let mut ue_point = Point::new(0.5, 0.5);
        ue_in_reach.add_trail(vec![ue_point]);

        let mut ue_out_reach = User::new(0);
        ue_point = Point::new(-1., -1.);
        ue_out_reach.add_trail(vec![ue_point]);

        mobile_network.add_g_node_bs(&mut vec![gnb1, gnb2]);
        mobile_network.add_ues(&mut vec![ue_in_reach, ue_out_reach]);

        mobile_network.check_ues();

        assert_eq!(mobile_network.ues[0].state, UserState::InReach);
        assert_eq!(mobile_network.ues[1].state, UserState::OutOfReach);
    }
}
