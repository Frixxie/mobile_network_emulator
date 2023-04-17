use url::Url;

#[derive(Debug, Clone)]
pub struct Application {
    url: Url,
}

impl Application {
    pub fn new(url: Url) -> Self {
        Application { url }
    }

    pub fn url(&self) -> &Url {
        &self.url
    }
}
