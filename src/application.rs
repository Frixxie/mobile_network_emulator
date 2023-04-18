use url::Url;

#[derive(Debug, Clone)]
pub struct Application {
    url: Url,
    id: u32,
}

impl Application {
    pub fn new(url: Url, id: u32) -> Self {
        Application { url, id }
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn id(&self) -> &u32 {
        &self.id
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    pub fn create_application() {
        let application = Application::new(Url::parse("http://fasteraune.com").unwrap(), 0);
        let url = application.url();
        let id = application.id();

        assert_eq!(url, &Url::parse("http://fasteraune.com").unwrap());
        assert_eq!(*id, 0);
    }
}
