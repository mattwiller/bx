use super::Error;
use bytes::Bytes;
use futures::stream::TryStream;
use reqwest::multipart::Form as MultipartForm;
use reqwest::{
    Client as ReqwestClient, Method as ReqwestMethod,
    Request as ReqwestRequest, RequestBuilder, Response as ReqwestResponse,
};
use serde_json::Value;
use std::convert::TryFrom;

pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    OPTIONS,
    DELETE,
}

impl From<HTTPMethod> for ReqwestMethod {
    fn from(method: HTTPMethod) -> ReqwestMethod {
        match method {
            HTTPMethod::GET => ReqwestMethod::GET,
            HTTPMethod::POST => ReqwestMethod::POST,
            HTTPMethod::PUT => ReqwestMethod::PUT,
            HTTPMethod::OPTIONS => ReqwestMethod::OPTIONS,
            HTTPMethod::DELETE => ReqwestMethod::DELETE,
        }
    }
}

pub enum Body {
    Empty,
    JSON(Value),
    Multipart(MultipartBody),
}

// impl From<Body> for ReqwestBody {
//     fn from(body: Body) -> ReqwestBody {
//         match body {
//             Body::Empty => ReqwestBody::from(""),
//             Body::JSON(json) => ReqwestBody::from(json.to_string()),
//         }
//     }
// }

#[derive(Default)]
pub struct MultipartBody {
    form: MultipartForm,
}

impl MultipartBody {
    pub fn new() -> MultipartBody {
        MultipartBody {
            form: MultipartForm::new(),
        }
    }

    pub fn with_text_part(mut self, name: &str, body: &str) -> MultipartBody {
        self.form = self.form.text(name.to_owned(), body.to_owned());
        self
    }

    pub fn with_stream_part<S>(mut self, name: &str, body: S) -> MultipartBody
    where
        S: TryStream + Send + Sync + 'static,
        S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
        Bytes: From<S::Ok>,
    {
        let file_part = reqwest::multipart::Part::stream(reqwest::Body::wrap_stream(body))
            .file_name("UNUSED")
            .mime_str("application/octet-stream")
            .unwrap();

        self.form = self.form.part(name.to_owned(), file_part);
        self
    }
}

pub struct Request {
    req: RequestBuilder,
}

impl Request {
    fn new(req: RequestBuilder) -> Request {
        Request { req }
    }

    pub fn with_body(mut self, body: Body) -> Request {
        self.req = match body {
            Body::Empty => self.req,
            Body::JSON(json) => self.req.body(json.to_string()),
            Body::Multipart(body) => self.req.multipart(body.form),
        };
        self
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Request {
        self.req = self.req.header(key, value);
        self
    }
}

impl TryFrom<Request> for ReqwestRequest {
    type Error = Error;
    fn try_from(request: Request) -> Result<ReqwestRequest, Error> {
        Ok(request.req.build()?)
    }
}

pub struct Response {
    res: ReqwestResponse,
}

impl Response {
    pub async fn deserialize<T: serde::de::DeserializeOwned>(self) -> Result<T, Error> {
        self.res.json().await.map_err(Error::from)
    }

    pub async fn chunk(&mut self) -> Result<Option<Bytes>, Error> {
        Ok(self.res.chunk().await?)
    }
}

impl From<ReqwestResponse> for Response {
    fn from(res: ReqwestResponse) -> Response {
        Response { res }
    }
}

pub struct NetworkAgent {
    http_client: ReqwestClient,
}

impl NetworkAgent {
    pub fn new() -> NetworkAgent {
        NetworkAgent {
            http_client: ReqwestClient::default(),
        }
    }

    pub fn start_request(&self, method: HTTPMethod, url: &str) -> Request {
        let req = self.http_client.request(method.into(), url);
        Request::new(req)
    }

    pub async fn send_request(&self, request: Request) -> Result<Response, Error> {
        let res = self
            .http_client
            .execute(ReqwestRequest::try_from(request)?)
            .await?;
        Ok(Response::from(res))
    }
}
