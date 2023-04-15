use geo::{MultiPoint, Point};
use rand::Rng;

#[derive(Debug, PartialEq, Clone)]
pub struct User {
    id: u32,
    posititon: usize,
    path: Option<MultiPoint>,
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
                self.posititon = self.posititon % path.iter().count();
                Some(self.posititon)
            }
            None => None,
        }
    }

    pub fn generate_user_trail(bounds: (f64, f64), start_pos: Point, length: usize) -> MultiPoint {
        let mut rng = rand::thread_rng();
        let mut res = Vec::new();
        res.push(start_pos);
        for i in 0..length - 1 {
            let diff = (rng.gen_range(-1.0..1.), rng.gen_range(-1.0..1.));
            let point = Point::new(
                (res[i].x() + diff.0) % bounds.0,
                (res[i].y() + diff.1) % bounds.1,
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
    fn path() {
        let mut user = User::new(0);
        let path = MultiPoint(vec![Point::new(0.0, 0.1), Point::new(1., 1.)]);

        let current_pos = user.current_pos();
        assert_eq!(current_pos, None);

        let next_pos = user.next_pos();
        assert_eq!(next_pos, None);

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
    fn generate_trail() {
        let trail = User::generate_user_trail((100., 100.), (50., 50.).into(), 1 << 7);
        assert_eq!(trail.iter().count(), 1 << 7);
    }
}
