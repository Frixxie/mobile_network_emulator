use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Application {
    id: u32,
}

impl Application {
    pub fn new(id: u32) -> Self {
        Application { id }
    }

    pub fn id(&self) -> &u32 {
        &self.id
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    pub fn new() {
        let application = Application::new(0);
        let id = application.id();

        assert_eq!(*id, 0);
    }
}
