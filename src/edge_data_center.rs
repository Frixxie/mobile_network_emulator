use geo::Point;
use url::Url;

use crate::{application::Application, application_runtime::ApplicationRuntime};

pub struct EdgeDataCenter {
    application_runtime: ApplicationRuntime,
    name: String,
    posititon: Point,
}

impl EdgeDataCenter {
    pub fn new(name: String, position: Point) -> Self {
        todo!();
    }

    pub fn add_application(&mut self, application: Application) -> Url {
        todo!();
    }

    pub fn delete_application(&mut self, application: &Application) -> Application {
        todo!();
    }

    pub fn use_application(&mut self, application: &Application) -> u32 {
        todo!();
    }

    pub fn contains_application(&self, url: &Url) -> bool {
        self.application_runtime.contains_application(url)
    }
}
