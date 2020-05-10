#![warn(rust_2018_idioms)]
#![deny(clippy::all)]

use reqwest::Client as HttpClient;
use reqwest::Response;
use serde::Serialize;

use super::auth::{Auth, SingleTokenAuth};
use super::file::File;
use super::user::User;
use super::Error;

pub struct Client {
    auth: Box<dyn Auth>,
    client: HttpClient,
}

impl Client {
    pub fn new(token: String) -> Client {
        let client = HttpClient::new();
        Client {
            auth: Box::from(SingleTokenAuth::new(token)),
            client,
        }
    }

    pub async fn get(&mut self, url: &str) -> Result<Response, Error> {
        let access_token = &self.auth.token().await?;
        let request = self.client.get(url).bearer_auth(access_token.as_str());
        request.send().await.map_err(Error::from)
    }

    pub async fn put<T: Serialize>(&mut self, url: &str, body: T) -> Result<Response, Error> {
        let access_token = &self.auth.token().await?;
        let request = self
            .client
            .put(url)
            .bearer_auth(access_token.as_str())
            .json(&body);
        request.send().await.map_err(Error::from)
    }

    pub async fn delete(&mut self, url: &str) -> Result<Response, Error> {
        let access_token = &self.auth.token().await?;
        let request = self.client.delete(url).bearer_auth(access_token.as_str());
        request.send().await.map_err(Error::from)
    }

    pub async fn multipart_upload<'a>(
        &mut self,
        url: &str,
        form: reqwest::multipart::Form,
    ) -> Result<Response, Error> {
        let access_token = &self.auth.token().await?;
        let request = self
            .client
            .post(url)
            .bearer_auth(access_token.as_str())
            .multipart(form);
        request.send().await.map_err(Error::from)
    }

    pub async fn get_file(&mut self, id: &str) -> Result<File, Error> {
        let url = format!("https://api.box.com/2.0/files/{}", id);
        let resp = self.get(&url).await?;

        let file: File = serde_json::from_str(&resp.text().await?)?;

        Ok(file)
    }

    pub async fn get_user(&mut self, id: &str) -> Result<User, Error> {
        let url = format!("https://api.box.com/2.0/users/{}", id);
        let resp = self.get(&url).await?;

        let user: User = serde_json::from_str(&resp.text().await?)?;

        Ok(user)
    }
}
