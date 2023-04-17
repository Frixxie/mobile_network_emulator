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
        application: Application,
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

    pub fn use_application(&mut self, application: &Application) -> u32 {
        todo!();
    }

    pub fn contains_application(&self, url: &Url) -> bool {
        self.applications
            .iter()
            .filter(|(application, _usages)| application.url() == url)
            .count()
            > 0
    }
}
