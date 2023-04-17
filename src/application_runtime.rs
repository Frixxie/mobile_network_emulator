use url::Url;

use crate::application::Application;

#[derive(Debug, Serialize)]
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
        todo!();
    }

    pub fn add_application(
        &mut self,
        application: Application,
    ) -> Result<(), ApplicationRuntimeError> {
        if self.applications.contains(&application) {
            return Err(ApplicationRuntimeError::new(format!(
                "Application {} already exists",
                application.get_name()
            )));
        }
        self.applications.push(application);
        Ok(())
    }

    pub fn remove_application(
        &mut self,
        application: Application,
    ) -> Result<(), ApplicationRuntimeError> {
        for (i, current_application) in self.applications.iter_mut().enumerate() {
            if *current_application == application {
                self.applications.remove(i);
                return Ok(());
            }
        }
        Err(ApplicationRuntimeError::new(format!(
            "Application {} does not exist",
            application.get_name()
        )))
    }

    pub fn use_application(&mut self, application: &Application) -> u32 {
        todo!();
    }

    pub fn contains_application(&mut self, url: Url) -> bool {
        self.applications
            .iter()
            .filter(|(application, usages)| application.url() == url)
            .count()
            > 0
    }
}
