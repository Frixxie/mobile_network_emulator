use serde::{ser::SerializeStruct, Serialize};
use url::Url;

#[derive(Debug)]
pub struct Application {
    url: Url,
    id: u32,
}

impl Serialize for Application {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Application", 2)?;
        state.serialize_field("url", self.url.as_str())?;
        state.serialize_field("id", self.id())?;
        state.end()
    }
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

impl Clone for Application {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            url: self.url.clone(),
        }
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
