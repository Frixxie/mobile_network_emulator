use std::{f64::consts::PI, fmt::Display, ops::Range};

use geo::{MultiPoint, Point};
use rand::{prelude::Distribution, rngs::ThreadRng};
use rand_distr::Normal;
use serde::{ser::SerializeStruct, Serialize};

const VELOCITY: f64 = 1.0;

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
        state.serialize_field("id", &self.id)?;
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

    fn next_dir(rng: &mut ThreadRng, mut last_x: f64, mut last_y: f64) -> (f64, f64) {
        let length = (last_x * last_x + last_y * last_y).sqrt();
        last_x /= length;
        last_y /= length;

        let normal = Normal::new(0.0, PI / 3.0).unwrap();

        let mut alpha = normal.sample(rng);
        alpha = alpha.clamp(-PI, PI);

        (
            alpha.cos() * last_x + alpha.sin() * last_y,
            alpha.cos() * last_y - alpha.sin() * last_x,
        )
    }

    pub fn generate_user_path(bounds: &Range<f64>, start_pos: Point, length: usize) -> MultiPoint {
        let mut rng = rand::thread_rng();
        let mut res = Vec::new();
        res.push(start_pos);
        let mut diff = (1.0, 0.0);
        for i in 0..length - 1 {
            diff = Self::next_dir(&mut rng, diff.0, diff.1);
            let point = Point::new(
                (res[i].x() + diff.0 * VELOCITY) % bounds.start,
                (res[i].y() + diff.1 * VELOCITY) % bounds.end,
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
        for point in &path {
            println!("{},{}", point.x(), point.y());
        }
        assert_eq!(path.iter().count(), 1 << 7);
        assert!(false);
    }
}
