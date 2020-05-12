#![warn(rust_2018_idioms)]
#![deny(clippy::all)]

use serde::Serialize;

use super::auth::{Auth, SingleTokenAuth};
use super::collection::Collection;
use super::file::File;
use super::operations::{FileOperation, UserOperation};
use super::{Body, HTTPMethod, MultipartBody, NetworkAgent, Request, Response, SDKError};
use serde_json::json;
use std::path::Path;
use tokio::fs;
use tokio_util::codec::{BytesCodec, FramedRead};

pub struct Client {
    auth: Box<dyn Auth>,
    network: NetworkAgent,
}

impl Client {
    pub fn new(token: String) -> Client {
        Client {
            auth: Box::from(SingleTokenAuth::new(token)),
            network: NetworkAgent::new(),
        }
    }

    async fn make_request(&mut self, request: Request) -> Result<Response, SDKError> {
        let mut request = request;

        let access_token = &self.auth.token().await?;
        request = request.with_header(
            "Authorization",
            &format!("Bearer {}", access_token.as_str()),
        );
        self.network.send_request(request).await
    }

    pub async fn get(&mut self, url: &str) -> Result<Response, SDKError> {
        let request = self.network.start_request(HTTPMethod::GET, url);
        self.make_request(request).await
    }

    pub async fn put<T: Serialize>(&mut self, url: &str, body: T) -> Result<Response, SDKError> {
        let request = self
            .network
            .start_request(HTTPMethod::PUT, url)
            .with_body(Body::JSON(serde_json::to_value(body)?));
        self.make_request(request).await
    }

    pub async fn delete(&mut self, url: &str) -> Result<Response, SDKError> {
        let request = self.network.start_request(HTTPMethod::DELETE, url);
        self.make_request(request).await
    }

    pub async fn multipart_upload<'a>(
        &mut self,
        url: &str,
        body: MultipartBody,
    ) -> Result<Response, SDKError> {
        let request = self
            .network
            .start_request(HTTPMethod::POST, url)
            .with_body(Body::Multipart(body));
        self.make_request(request).await
    }

    pub fn file<'a>(&'a mut self, id: &'a str) -> FileOperation<'a> {
        FileOperation::new(id, self)
    }

    pub fn user<'a>(&'a mut self, id: &'a str) -> UserOperation<'a> {
        UserOperation::new(id, self)
    }

    pub async fn upload_file(&mut self, path: &Path, folder_id: &str) -> Result<File, SDKError> {
        let file = fs::File::open(path).await?;
        let stream = FramedRead::new(file, BytesCodec::new());

        let filename = path.file_name().unwrap().to_str();
        let attributes_json = json!({
            "name": filename,
            "parent": {
                "id": folder_id
            }
        })
        .to_string();

        let form = MultipartBody::new()
            .with_text_part("attributes", &attributes_json)
            .with_stream_part("file", stream);

        let url = "https://upload.box.com/api/2.0/files/content";

        let response = self.multipart_upload(&url, form).await?;
        let data: Collection<File> = response.deserialize().await?;
        Ok(data.entries[0].to_owned())
    }
}
