#![warn(rust_2018_idioms)]
#![deny(clippy::all)]

use reqwest::{Response as ReqwestResponse, Client as ReqwestClient};
use serde::Serialize;

use super::auth::{Auth, SingleTokenAuth};
use super::file::File;
use super::user::User;
use super::{Error, NetworkAgent, Request, Response, Body, HTTPMethod};

pub struct Client {
    auth: Box<dyn Auth>,
    client: ReqwestClient,
    network: NetworkAgent,
}

impl Client {
    pub fn new(token: String) -> Client {
        let client = ReqwestClient::default();
        Client {
            auth: Box::from(SingleTokenAuth::new(token)),
            client,
            network: NetworkAgent::new(),
        }
    }

    async fn make_request(&mut self, request: Request) -> Result<Response, Error> {
        let access_token = &self.auth.token().await?;
        request.add_header("Authorization", &format!("Bearer {}", access_token.as_str()));
        self.network.send_request(request).await
    }

    pub async fn get(&mut self, url: &str) -> Result<Response, Error> {
        let mut request = self.network.start_request(HTTPMethod::GET, url);
        self.make_request(request).await
    }

    pub async fn put<T: Serialize>(&mut self, url: &str, body: T) -> Result<Response, Error> {
        let mut request = self.network.start_request(HTTPMethod::PUT, url);
        request.set_body(Body::JSON(serde_json::to_value(body)?));
        self.make_request(request).await
    }

    pub async fn delete(&mut self, url: &str) -> Result<Response, Error> {
        let request = self.network.start_request(HTTPMethod::DELETE, url);
        self.make_request(request).await
    }

    pub async fn multipart_upload<'a>(
        &mut self,
        url: &str,
        form: reqwest::multipart::Form,
    ) -> Result<ReqwestResponse, Error> {
        // let access_token = &self.auth.token().await?;
        // let request = self
        //     .client
        //     .post(url)
        //     .bearer_auth(access_token.as_str())
        //     .multipart(form);
        // request.send().await.map_err(Error::from)

        let request = self.network.start_request(HTTPMethod::POST, url);
        request.set_body(Body::Multipart())
    }

    pub async fn get_file(&mut self, id: &str) -> Result<File, Error> {
        let url = format!("https://api.box.com/2.0/files/{}", id);
        let response = self.get(&url).await?;

        let file: File = response.deserialize().await?;

        Ok(file)
    }

    pub async fn get_user(&mut self, id: &str) -> Result<User, Error> {
        let url = format!("https://api.box.com/2.0/users/{}", id);
        let response = self.get(&url).await?;

        let user: User = response.deserialize().await?;

        Ok(user)
    }
}
