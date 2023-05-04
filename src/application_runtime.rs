use std::{
    error::Error,
    fmt::{Display, Formatter},
    net::IpAddr,
};

use crate::application::Application;

#[derive(Debug)]
pub struct ApplicationRuntimeError {
    message: String,
}

impl ApplicationRuntimeError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for ApplicationRuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ApplicationRuntimeError {}

#[derive(Debug, Clone)]
pub struct ApplicationRuntime {
    applications: Vec<Application>,
}

impl ApplicationRuntime {
    pub fn new() -> Self {
        ApplicationRuntime {
            applications: Vec::new(),
        }
    }

    pub fn add_application(
        &mut self,
        application: Application,
    ) -> Result<(), ApplicationRuntimeError> {
        if self.contains_application(application.id()) {
            return Err(ApplicationRuntimeError::new(
                "Application already exists".to_string(),
            ));
        }
        self.applications.push(application);
        Ok(())
    }

    pub fn remove_application(
        &mut self,
        application: &Application,
    ) -> Result<(), ApplicationRuntimeError> {
        for (i, current_application) in self.applications.iter_mut().enumerate() {
            if current_application.id() == application.id() {
                self.applications.remove(i);
                return Ok(());
            }
        }
        Err(ApplicationRuntimeError::new(
            "Application does not exist".to_string(),
        ))
    }

    pub fn use_application(
        &mut self,
        ip_addr: IpAddr,
        application: &Application,
    ) -> Result<u32, ApplicationRuntimeError> {
        for current_application in self.applications.iter_mut() {
            if current_application.id() == application.id() {
                current_application.add_use(ip_addr);
                return Ok(current_application.get_use(&ip_addr));
            }
        }
        Err(ApplicationRuntimeError::new(
            "Application does not exist".to_string(),
        ))
    }

    pub fn contains_application(&self, id: &u32) -> bool {
        match self
            .applications
            .iter()
            .find(|application| application.id() == id)
        {
            Some(_) => true,
            None => false,
        }
    }

    pub fn num_applications(&self) -> usize {
        self.applications.len()
    }

    pub fn get_applications(&self) -> Vec<&Application> {
        self.applications
            .iter()
            .map(|appliaction| appliaction)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use super::*;

    #[test]
    fn add_application() {
        let mut application_runtime = ApplicationRuntime::new();
        let application = Application::new(0);
        application_runtime.add_application(application).unwrap();
        assert_eq!(application_runtime.applications.len(), 1);
    }

    #[test]
    fn add_same_application_two_times_should_fail() {
        let mut application_runtime = ApplicationRuntime::new();
        let application = Application::new(0);
        application_runtime.add_application(application).unwrap();
        assert_eq!(application_runtime.applications.len(), 1);

        let application = Application::new(0);
        let res = application_runtime.add_application(application);
        assert!(res.is_err());
        assert_eq!(application_runtime.applications.len(), 1);
    }

    #[test]
    fn remove_application() {
        let mut application_runtime = ApplicationRuntime::new();
        let application = Application::new(0);
        application_runtime
            .add_application(application.clone())
            .unwrap();
        assert_eq!(application_runtime.applications.len(), 1);

        let res = application_runtime.remove_application(&application);
        assert!(res.is_ok());
        assert_eq!(application_runtime.applications.len(), 0);
    }

    #[test]
    fn remove_application_two_times() {
        let mut application_runtime = ApplicationRuntime::new();
        let application = Application::new(0);
        application_runtime
            .add_application(application.clone())
            .unwrap();
        assert_eq!(application_runtime.applications.len(), 1);

        application_runtime
            .remove_application(&application)
            .unwrap();
        assert_eq!(application_runtime.applications.len(), 0);
        let res = application_runtime.remove_application(&application);
        assert!(res.is_err());
    }

    #[test]
    fn use_application() {
        let mut application_runtime = ApplicationRuntime::new();
        let application = Application::new(0);
        application_runtime
            .add_application(application.clone())
            .unwrap();

        assert_eq!(application_runtime.applications.len(), 1);

        let ip_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

        application_runtime
            .use_application(ip_addr, &application)
            .unwrap();
        let application_use = application_runtime.get_applications()[0].get_use(&ip_addr);
        assert_eq!(application_use, 1);
    }

    #[test]
    fn use_application_when_application_does_not_exsist() {
        let mut application_runtime = ApplicationRuntime::new();
        let application = Application::new(0);

        let ip_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

        let res = application_runtime.use_application(ip_addr, &application);
        assert!(res.is_err());
    }

    #[test]
    fn num_applications() {
        let mut application_runtime = ApplicationRuntime::new();
        let application = Application::new(0);

        assert_eq!(application_runtime.applications.len(), 0);
        assert_eq!(application_runtime.num_applications(), 0);

        application_runtime.add_application(application).unwrap();

        assert_eq!(application_runtime.applications.len(), 1);
        assert_eq!(application_runtime.num_applications(), 1);
    }
}
