use std::{
    error::Error,
    fmt::{Display, Formatter},
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
    applications: Vec<(Application, u32)>,
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
        self.applications.push((application, 0));
        Ok(())
    }

    pub fn remove_application(
        &mut self,
        application: &Application,
    ) -> Result<(), ApplicationRuntimeError> {
        for (i, current_application) in self.applications.iter_mut().enumerate() {
            if current_application.0.id() == application.id() {
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
        application: &Application,
    ) -> Result<u32, ApplicationRuntimeError> {
        for current_application in self.applications.iter_mut() {
            if current_application.0.id() == application.id() {
                current_application.1 += 1;
                return Ok(current_application.1);
            }
        }
        Err(ApplicationRuntimeError::new(
            "Application does not exist".to_string(),
        ))
    }

    pub fn contains_application(&self, id: &u32) -> bool {
        self.applications
            .iter()
            .filter(|(application, _usages)| application.id() == id)
            .count()
            > 0
    }

    pub fn num_applications(&self) -> usize {
        self.applications.len()
    }

    pub fn get_applications(&self) -> Vec<&Application> {
        self.applications
            .iter()
            .map(|(appliaction, _id)| appliaction)
            .collect()
    }
}

#[cfg(test)]
mod tests {
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

        application_runtime.use_application(&application).unwrap();
        let application_use = application_runtime.applications.iter().last().unwrap().1;
        assert_eq!(application_use, 1);
    }

    #[test]
    fn use_application_when_application_does_not_exsist() {
        let mut application_runtime = ApplicationRuntime::new();
        let application = Application::new(0);

        let res = application_runtime.use_application(&application);
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
