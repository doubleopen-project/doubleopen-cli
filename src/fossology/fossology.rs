use reqwest;
use reqwest::blocking::Client;

pub struct Fossology {
    uri: String,
    token: String,
    client: Client,
}

impl Fossology {
    pub fn new(uri: &str, token: &str) -> Self {
        Self {
            uri: uri.to_owned(),
            token: token.to_owned(),
            client: Client::new(),
        }
    }

    pub fn version(&self) {
        let body = self
            .client
            .get(&format!("{}/version", self.uri))
            .bearer_auth(&self.token)
            .send()
            .unwrap()
            .text()
            .unwrap();
        println!("Fossology version: {}", body);
    }
}
