use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use geo::Point;
use serde::{ser::SerializeStruct, Serialize};
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

#[derive(Debug, Clone)]
pub struct EdgeDataCenter {
    application_runtime: ApplicationRuntime,
    id: usize,
    name: String,
    position: Point,
}

impl EdgeDataCenter {
    pub fn new(id: usize, name: &str, position: Point) -> Self {
        EdgeDataCenter {
            application_runtime: ApplicationRuntime::new(),
            id,
            name: name.to_string(),
            position,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
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

    pub fn get_applications(&self) -> Vec<&Application> {
        self.application_runtime.get_applications()
    }
}

impl Serialize for EdgeDataCenter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("EdgeDataCenter", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("x", &self.position.x())?;
        state.serialize_field("y", &self.position.y())?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_edge_datacenter() {
        let eds = EdgeDataCenter::new(0, "Fredrik's EdgeDataCenter", Point::new(0., 0.));
        assert_eq!(eds.name, "Fredrik's EdgeDataCenter");
        assert_eq!(eds.position, Point::new(0., 0.));
    }

    #[test]
    fn add_application() {
        let mut eds = EdgeDataCenter::new(0, "Fredrik's EdgeDataCenter", Point::new(0., 0.));
        let application = Application::new(Url::parse("http://fasteraune.com").unwrap(), 0);
        let res = eds.add_application(&application);
        assert!(res.is_ok());
        assert_eq!(eds.application_runtime.num_applications(), 1);
    }

    #[test]
    fn add_application_already_present_should_fail() {
        let mut eds = EdgeDataCenter::new(0, "Fredrik's EdgeDataCenter", Point::new(0., 0.));
        let application = Application::new(Url::parse("http://fasteraune.com").unwrap(), 0);
        let mut res = eds.add_application(&application);
        assert!(res.is_ok());
        assert_eq!(eds.application_runtime.num_applications(), 1);
        res = eds.add_application(&application);
        assert!(res.is_err());
    }

    #[test]
    fn remove_application() {
        let mut eds = EdgeDataCenter::new(0, "Fredrik's EdgeDataCenter", Point::new(0., 0.));
        let application = Application::new(Url::parse("http://fasteraune.com").unwrap(), 0);
        let res = eds.add_application(&application);
        assert!(res.is_ok());
        assert_eq!(eds.application_runtime.num_applications(), 1);

        let res = eds.remove_application(&application);
        assert!(res.is_ok());
        assert_eq!(eds.application_runtime.num_applications(), 0);
    }

    #[test]
    fn remove_application_two_times() {
        let mut eds = EdgeDataCenter::new(0, "Fredrik's EdgeDataCenter", Point::new(0., 0.));
        let application = Application::new(Url::parse("http://fasteraune.com").unwrap(), 0);
        let res = eds.add_application(&application);
        assert!(res.is_ok());
        assert_eq!(eds.application_runtime.num_applications(), 1);

        let res = eds.remove_application(&application);
        assert!(res.is_ok());
        assert_eq!(eds.application_runtime.num_applications(), 0);

        let res = eds.remove_application(&application);
        assert!(res.is_err());
        assert_eq!(eds.application_runtime.num_applications(), 0);
    }

    #[test]
    fn use_application() {
        let mut eds = EdgeDataCenter::new(0, "Fredrik's EdgeDataCenter", Point::new(0., 0.));
        let application = Application::new(Url::parse("http://fasteraune.com").unwrap(), 0);
        let res = eds.add_application(&application);

        assert!(res.is_ok());
        assert_eq!(eds.application_runtime.num_applications(), 1);

        let res = eds.use_application(&application);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1);
    }

    #[test]
    fn use_application_no_application_should_fail() {
        let mut eds = EdgeDataCenter::new(0, "Fredrik's EdgeDataCenter", Point::new(0., 0.));
        let application = Application::new(Url::parse("http://fasteraune.com").unwrap(), 0);

        let res = eds.use_application(&application);
        assert!(res.is_err());
    }

    #[test]
    fn contains_application() {
        let mut eds = EdgeDataCenter::new(0, "Fredrik's EdgeDataCenter", Point::new(0., 0.));
        let application = Application::new(Url::parse("http://fasteraune.com").unwrap(), 0);
        let res = eds.add_application(&application);

        assert!(res.is_ok());
        assert_eq!(eds.application_runtime.num_applications(), 1);

        let res = eds.contains_application(&Url::parse("http://fasteraune.com").unwrap());
        assert!(res);
    }

    #[test]
    fn not_contain_application() {
        let eds = EdgeDataCenter::new(0, "Fredrik's EdgeDataCenter", Point::new(0., 0.));

        let res = eds.contains_application(&Url::parse("http://fasteraune.com").unwrap());
        assert!(!res);
    }
}
