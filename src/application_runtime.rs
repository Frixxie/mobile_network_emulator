use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use url::Url;

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
        if self.contains_application(application.url()) {
            return Err(ApplicationRuntimeError::new(format!(
                "Application already exists",
            )));
        }
        self.applications.push((application, 0));
        Ok(())
    }

    pub fn remove_application(
        &mut self,
        application: &Application,
    ) -> Result<(), ApplicationRuntimeError> {
        for (i, current_application) in self.applications.iter_mut().enumerate() {
            if current_application.0.url() == application.url() {
                self.applications.remove(i);
                return Ok(());
            }
        }
        Err(ApplicationRuntimeError::new(format!(
            "Application does not exist",
        )))
    }

    pub fn use_application(
        &mut self,
        application: &Application,
    ) -> Result<u32, ApplicationRuntimeError> {
        for current_application in self.applications.iter_mut() {
            if current_application.0.url() == application.url() {
                current_application.1 += 1;
                return Ok(current_application.1);
            }
        }
        Err(ApplicationRuntimeError::new(format!(
            "Application does not exist",
        )))
    }

    pub fn contains_application(&self, url: &Url) -> bool {
        self.applications
            .iter()
            .filter(|(application, _usages)| application.url() == url)
            .count()
            > 0
    }

    pub fn num_applications(&self) -> usize {
        self.applications.iter().count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_application() {
        let mut application_runtime = ApplicationRuntime::new();
        let application = Application::new(Url::parse("https://fasteraune.com").unwrap(), 0);
        application_runtime.add_application(application).unwrap();
        assert_eq!(application_runtime.applications.iter().count(), 1);
    }

    #[test]
    fn add_same_application_two_times_should_fail() {
        let mut application_runtime = ApplicationRuntime::new();
        let application = Application::new(Url::parse("https://fasteraune.com").unwrap(), 0);
        application_runtime.add_application(application).unwrap();
        assert_eq!(application_runtime.applications.iter().count(), 1);

        let application = Application::new(Url::parse("https://fasteraune.com").unwrap(), 0);
        let res = application_runtime.add_application(application);
        assert_eq!(res.is_err(), true);
        assert_eq!(application_runtime.applications.iter().count(), 1);
    }

    #[test]
    fn remove_application() {
        let mut application_runtime = ApplicationRuntime::new();
        let application = Application::new(Url::parse("https://fasteraune.com").unwrap(), 0);
        application_runtime
            .add_application(application.clone())
            .unwrap();
        assert_eq!(application_runtime.applications.iter().count(), 1);

        let res = application_runtime.remove_application(&application);
        assert_eq!(res.is_ok(), true);
        assert_eq!(application_runtime.applications.iter().count(), 0);
    }

    #[test]
    fn remove_application_two_times() {
        let mut application_runtime = ApplicationRuntime::new();
        let application = Application::new(Url::parse("https://fasteraune.com").unwrap(), 0);
        application_runtime
            .add_application(application.clone())
            .unwrap();
        assert_eq!(application_runtime.applications.iter().count(), 1);

        application_runtime
            .remove_application(&application)
            .unwrap();
        assert_eq!(application_runtime.applications.iter().count(), 0);
        let res = application_runtime.remove_application(&application);
        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn use_application() {
        let mut application_runtime = ApplicationRuntime::new();
        let application = Application::new(Url::parse("https://fasteraune.com").unwrap(), 0);
        application_runtime
            .add_application(application.clone())
            .unwrap();

        assert_eq!(application_runtime.applications.iter().count(), 1);

        application_runtime.use_application(&application).unwrap();
        let application_use = application_runtime.applications.iter().last().unwrap().1;
        assert_eq!(application_use, 1);
    }

    #[test]
    fn use_application_when_application_does_not_exsist() {
        let mut application_runtime = ApplicationRuntime::new();
        let application = Application::new(Url::parse("https://fasteraune.com").unwrap(), 0);

        let res = application_runtime.use_application(&application);
        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn num_applications() {
        let mut application_runtime = ApplicationRuntime::new();
        let application = Application::new(Url::parse("https://fasteraune.com").unwrap(), 0);

        assert_eq!(application_runtime.applications.iter().count(), 0);
        assert_eq!(application_runtime.num_applications(), 0);

        application_runtime
            .add_application(application.clone())
            .unwrap();

        assert_eq!(application_runtime.applications.iter().count(), 1);
        assert_eq!(application_runtime.num_applications(), 1);
    }
}
