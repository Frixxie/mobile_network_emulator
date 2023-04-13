use geo::{Contains, Point, Rect};

use crate::user::User;

#[derive(Clone, Debug)]
pub struct GNodeB {
    id: u32,
    pos: Point,
    cell: Rect,
}

impl GNodeB {
    pub fn new(id: u32, pos: Point, cell: Rect) -> Self {
        GNodeB { id, pos, cell }
    }
}

impl Contains<User> for GNodeB {
    fn contains(&self, rhs: &User) -> bool {
        dbg!(self, rhs);
        self.cell.contains(&rhs.current_pos().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo::coord;

    #[test]
    fn contains() {
        let rect = Rect::new(coord! { x: 0., y: 0.}, coord! { x: 1., y: 1.});
        let gnb = GNodeB::new(1, (0.5, 0.5).into(), rect);
        let mut ue_inside = User::new(0);
        let ue_point = Point::new(0.5, 0.5);
        ue_inside.add_trail(vec![ue_point]);

        let mut ue_outside = User::new(0);
        let ue_point = Point::new(-1., -1.);
        ue_outside.add_trail(vec![ue_point]);

        if !gnb.contains(&ue_inside) || gnb.contains(&ue_outside) {
            assert!(false);
        }
    }
}
