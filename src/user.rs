use std::{f64::consts::PI, fmt::Display, ops::Range};

use geo::Point;
use rand::{prelude::Distribution, rngs::ThreadRng};
use rand_distr::Normal;
use serde::{ser::SerializeStruct, Serialize};

#[derive(Debug, PartialEq, Clone)]
pub struct User {
    id: u32,
    posititon: Point,
    velocity: f64,
    bounds: Range<f64>,
    current_direction: (f64, f64),
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("User", 2)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("x", &self.posititon.x())?;
        state.serialize_field("y", &self.posititon.y())?;
        state.end()
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "id: {}, ({},{})",
            self.id,
            self.posititon.x(),
            self.posititon.y()
        ))
    }
}

impl User {
    pub fn new(id: u32, posititon: Point, velocity: f64, bounds: &Range<f64>) -> Self {
        User {
            id,
            posititon,
            velocity,
            bounds: bounds.clone(),
            current_direction: (1.0, 0.0),
        }
    }

    pub fn current_pos(&self) -> Point {
        self.posititon
    }

    pub fn next_pos(&mut self) -> Point {
        let mut rng = rand::thread_rng();
        let diff = Self::next_dir(&mut rng, self.current_direction.0, self.current_direction.1);
        self.posititon = Point::new(
            (self.posititon.x() + diff.0 * self.velocity) % self.bounds.start,
            (self.posititon.y() + diff.1 * self.velocity) % self.bounds.end,
        );
        self.current_direction = diff;
        self.posititon
    }

    fn next_dir(rng: &mut ThreadRng, mut last_x: f64, mut last_y: f64) -> (f64, f64) {
        let length = (last_x * last_x + last_y * last_y).sqrt();
        last_x /= length;
        last_y /= length;

        let normal = Normal::new(0.0, PI / 16.0).unwrap();

        let mut alpha = normal.sample(rng);
        alpha = alpha.clamp(-PI, PI);

        (
            alpha.cos() * last_x + alpha.sin() * last_y,
            alpha.cos() * last_y - alpha.sin() * last_x,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let user = User::new(0, Point::new(0.0, 0.1), 1., &(-500.0..500.));

        let res = "{\"id\":0,\"x\":0.0,\"y\":0.1}";

        let serialized = serde_json::to_string(&user).unwrap();
        assert_eq!(serialized, res);
    }
}
