#![warn(rust_2018_idioms)]
#![deny(clippy::all)]

use reqwest::Client as HttpClient;
use reqwest::Response;
use serde::Serialize;

use super::file::File;

pub struct Client {
    token: String,
    client: HttpClient,
}

impl Client {
    pub fn new(token: String) -> Client {
        let client = HttpClient::new();
        Client { token, client }
    }

    pub async fn get(&self, url: &str) -> Result<Response, reqwest::Error> {
        let request = self.client.get(url).bearer_auth(&self.token);
        request.send().await
    }

    pub async fn put<T: Serialize>(&self, url: &str, body: T) -> Result<Response, reqwest::Error> {
        let request = self.client.put(url).bearer_auth(&self.token).json(&body);
        request.send().await
    }

    pub async fn delete(&self, url: &str) -> Result<Response, reqwest::Error> {
        let request = self.client.delete(url).bearer_auth(&self.token);
        request.send().await
    }

    pub async fn multipart_upload<'a>(
        &self,
        url: &str,
        form: reqwest::multipart::Form,
    ) -> Result<Response, reqwest::Error> {
        let request = self
            .client
            .post(url)
            .bearer_auth(&self.token)
            .multipart(form);

        request.send().await
    }

    pub async fn get_file(&self, id: &str) -> Result<File, Box<dyn std::error::Error>> {
        let url = format!("https://api.box.com/2.0/files/{}", id);
        let resp = self.get(&url).await?;

        let file: File = serde_json::from_str(&resp.text().await?)?;

        Ok(file)
    }
}
