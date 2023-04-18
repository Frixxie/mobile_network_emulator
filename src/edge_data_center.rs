use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use geo::Point;
use url::Url;

use crate::{application::Application, application_runtime::ApplicationRuntime};

#[derive(Debug)]
pub struct EdgeDataCenterError {
    message: String,
}

impl EdgeDataCenterError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for EdgeDataCenterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for EdgeDataCenterError {}

pub struct EdgeDataCenter {
    application_runtime: ApplicationRuntime,
    name: String,
    position: Point,
}

impl EdgeDataCenter {
    pub fn new(name: &str, position: Point) -> Self {
        EdgeDataCenter {
            application_runtime: ApplicationRuntime::new(),
            name: name.to_string(),
            position,
        }
    }

    pub fn add_application(
        &mut self,
        application: &Application,
    ) -> Result<Url, EdgeDataCenterError> {
        match self
            .application_runtime
            .add_application(application.clone())
            .map_err(|err| EdgeDataCenterError::new(format!("{}", err)))
        {
            Ok(_) => Ok(application.url().clone()),
            Err(err) => Err(err),
        }
    }

    pub fn remove_application(
        &mut self,
        application: &Application,
    ) -> Result<(), EdgeDataCenterError> {
        match self
            .application_runtime
            .remove_application(application)
            .map_err(|err| EdgeDataCenterError::new(format!("{}", err)))
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub fn use_application(
        &mut self,
        application: &Application,
    ) -> Result<u32, EdgeDataCenterError> {
        self.application_runtime
            .use_application(application)
            .map_err(|err| EdgeDataCenterError::new(format!("{}", err)))
    }

    pub fn contains_application(&self, url: &Url) -> bool {
        self.application_runtime.contains_application(url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_edge_datacenter() {
        let eds = EdgeDataCenter::new("Fredrik's EdgeDataCenter", Point::new(0., 0.));
        assert_eq!(eds.name, "Fredrik's EdgeDataCenter");
        assert_eq!(eds.position, Point::new(0., 0.));
    }

    #[test]
    fn add_application() {
        let mut eds = EdgeDataCenter::new("Fredrik's EdgeDataCenter", Point::new(0., 0.));
        let application = Application::new(Url::parse("http://fasteraune.com").unwrap(), 0);
        let res = eds.add_application(&application);
        assert_eq!(res.is_ok(), true);
        assert_eq!(eds.application_runtime.num_applications(), 1);
    }

    #[test]
    fn add_application_already_present_should_fail() {
        let mut eds = EdgeDataCenter::new("Fredrik's EdgeDataCenter", Point::new(0., 0.));
        let application = Application::new(Url::parse("http://fasteraune.com").unwrap(), 0);
        let mut res = eds.add_application(&application);
        assert_eq!(res.is_ok(), true);
        assert_eq!(eds.application_runtime.num_applications(), 1);
        res = eds.add_application(&application);
        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn remove_application() {
        let mut eds = EdgeDataCenter::new("Fredrik's EdgeDataCenter", Point::new(0., 0.));
        let application = Application::new(Url::parse("http://fasteraune.com").unwrap(), 0);
        let res = eds.add_application(&application);
        assert_eq!(res.is_ok(), true);
        assert_eq!(eds.application_runtime.num_applications(), 1);

        let res = eds.remove_application(&application);
        assert_eq!(res.is_ok(), true);
        assert_eq!(eds.application_runtime.num_applications(), 0);
    }

    #[test]
    fn remove_application_two_times() {
        let mut eds = EdgeDataCenter::new("Fredrik's EdgeDataCenter", Point::new(0., 0.));
        let application = Application::new(Url::parse("http://fasteraune.com").unwrap(), 0);
        let res = eds.add_application(&application);
        assert_eq!(res.is_ok(), true);
        assert_eq!(eds.application_runtime.num_applications(), 1);

        let res = eds.remove_application(&application);
        assert_eq!(res.is_ok(), true);
        assert_eq!(eds.application_runtime.num_applications(), 0);

        let res = eds.remove_application(&application);
        assert_eq!(res.is_err(), true);
        assert_eq!(eds.application_runtime.num_applications(), 0);
    }

    #[test]
    fn use_application() {
        let mut eds = EdgeDataCenter::new("Fredrik's EdgeDataCenter", Point::new(0., 0.));
        let application = Application::new(Url::parse("http://fasteraune.com").unwrap(), 0);
        let res = eds.add_application(&application);

        assert_eq!(res.is_ok(), true);
        assert_eq!(eds.application_runtime.num_applications(), 1);

        let res = eds.use_application(&application);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), 1);
    }

    #[test]
    fn use_application_no_application_should_fail() {
        let mut eds = EdgeDataCenter::new("Fredrik's EdgeDataCenter", Point::new(0., 0.));
        let application = Application::new(Url::parse("http://fasteraune.com").unwrap(), 0);

        let res = eds.use_application(&application);
        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn contains_application() {
        let mut eds = EdgeDataCenter::new("Fredrik's EdgeDataCenter", Point::new(0., 0.));
        let application = Application::new(Url::parse("http://fasteraune.com").unwrap(), 0);
        let res = eds.add_application(&application);

        assert_eq!(res.is_ok(), true);
        assert_eq!(eds.application_runtime.num_applications(), 1);

        let res = eds.contains_application(&Url::parse("http://fasteraune.com").unwrap());
        assert_eq!(res, true);
    }

    #[test]
    fn not_contain_application() {
        let eds = EdgeDataCenter::new("Fredrik's EdgeDataCenter", Point::new(0., 0.));

        let res = eds.contains_application(&Url::parse("http://fasteraune.com").unwrap());
        assert_eq!(res, false);
    }
}
