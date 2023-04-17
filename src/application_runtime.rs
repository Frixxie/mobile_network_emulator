use crate::application::Application;

pub struct ApplicationRuntime {
    applications: Vec<(Application, u32)>,
}

impl ApplicationRuntime {
    pub fn new() -> Self {
        todo!();
    }

    pub fn add_application(application: Application) {
        todo!();
    }

    pub fn delete_application(application: &Application) -> Application {
        todo!();
    }

    pub fn use_application(application: &Application) -> u32 {
        todo!();
    }
}
