use std::{fmt::Display, ops::Range};

use geo::{MultiPoint, Point};
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};
use serde::{ser::SerializeStruct, Serialize};

#[derive(Debug, PartialEq, Clone)]
pub struct User {
    id: u32,
    posititon: usize,
    path: Option<MultiPoint>,
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let pos = match self.current_pos() {
            Some(point) => point,
            None => panic!("User has no added path"),
        };
        let mut state = serializer.serialize_struct("User", 2)?;
        state.serialize_field("x", &pos.x())?;
        state.serialize_field("y", &pos.y())?;
        state.end()
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pos = match self.current_pos() {
            Some(pos) => pos,
            None => panic!("User has no added path"),
        };
        f.write_str(&format!("id: {}, ({},{})", self.id, pos.x(), pos.y()))
    }
}

impl User {
    pub fn new(id: u32) -> Self {
        User {
            id,
            posititon: 0,
            path: None,
        }
    }

    pub fn add_path(&mut self, path: MultiPoint) {
        self.path = Some(path);
    }

    pub fn current_pos(&self) -> Option<Point> {
        match &self.path {
            Some(path) => path.iter().nth(self.posititon).copied(),
            None => None,
        }
    }

    pub fn next_pos(&mut self) -> Option<usize> {
        match &self.path {
            Some(path) => {
                self.posititon += 1;
                self.posititon %= path.iter().count();
                Some(self.posititon)
            }
            None => None,
        }
    }

    fn next_dir(rng: &mut ThreadRng, last_x: i8, last_y: i8) -> (i8, i8) {
        let possible_dir = vec![-1, 0, 1];
        let mut x = last_x;
        let mut y = last_y;
        while x == last_x && y == last_y {
            x = *possible_dir.choose(rng).unwrap();
            y = *possible_dir.choose(rng).unwrap();
        }
        (x.clone(), y.clone())
    }

    pub fn generate_user_path(bounds: &Range<f64>, start_pos: Point, length: usize) -> MultiPoint {
        let mut rng = rand::thread_rng();
        let mut res = Vec::new();
        res.push(start_pos);
        let mut diff = (0, 0);
        for i in 0..length - 1 {
            diff = Self::next_dir(&mut rng, diff.0, diff.1);
            let point = Point::new(
                (res[i].x() + f64::from(diff.0)) % bounds.start,
                (res[i].y() + f64::from(diff.1)) % bounds.end,
            );
            res.push(point)
        }
        MultiPoint(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_path_current_pos_next_pos() {
        let mut user = User::new(0);
        let path = MultiPoint(vec![Point::new(0.0, 0.1), Point::new(1., 1.)]);

        user.add_path(path);

        let current_pos = user.current_pos().unwrap();
        assert_eq!(current_pos, Point::new(0.0, 0.1));

        let next_pos = user.next_pos().unwrap();
        assert_eq!(next_pos, 1);
        let current_pos = user.current_pos().unwrap();
        assert_eq!(current_pos, Point::new(1., 1.));

        let next_pos = user.next_pos().unwrap();
        assert_eq!(next_pos, 0);
        let current_pos = user.current_pos().unwrap();
        assert_eq!(current_pos, Point::new(0.0, 0.1));
    }

    #[test]
    fn serialize() {
        let mut user = User::new(0);
        let path = MultiPoint(vec![Point::new(0.0, 0.1), Point::new(1., 1.)]);

        let res = "{\"x\":0.0,\"y\":0.1}";

        user.add_path(path);
        let serialized = serde_json::to_string(&user).unwrap();
        assert_eq!(serialized, res);
    }

    #[test]
    fn current_pos_next_pos_should_fail() {
        let mut user = User::new(0);

        let current_pos = user.current_pos();
        assert_eq!(current_pos, None);

        let next_pos = user.next_pos();
        assert_eq!(next_pos, None);
    }

    #[test]
    fn generate_path() {
        let path = User::generate_user_path(&(100.0..100.), (50., 50.).into(), 1 << 7);
        assert_eq!(path.iter().count(), 1 << 7);
    }
}
