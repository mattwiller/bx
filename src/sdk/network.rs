use reqwest::{Client as ReqwestClient, Request as ReqwestRequest, Response as ReqwestResponse, Method as ReqwestMethod, Body as ReqwestBody, RequestBuilder};
use reqwest::header::{HeaderValue, HeaderName};
use serde_json::Value;
use std::convert::TryFrom;
use super::Error;

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
    Multipart(MultipartBody)
}

// impl From<Body> for ReqwestBody {
//     fn from(body: Body) -> ReqwestBody {
//         match body {
//             Body::Empty => ReqwestBody::from(""),
//             Body::JSON(json) => ReqwestBody::from(json.to_string()),
//         }
//     }
// }

pub struct MultipartBody {
    inner: reqwest::multipart::Form,
}

impl MultipartBody {

}

pub struct Request {
    req: RequestBuilder,
}

impl Request {
    fn new(req: RequestBuilder) -> Request {
        Request { req }
    }

    pub fn set_body(&mut self, body: Body) {
        match body {
            Body::Empty => {},
            Body::JSON(json) => { self.req.body(json.to_string()); },
            Body::Multipart(form) => { self.req.multipart(form.inner); },
        }
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.req.header(key, value);
    }
}

impl From<Request> for ReqwestRequest {
    fn from(request: Request) -> Result<ReqwestRequest, Error> {
        Ok(request.req.build()?)
    }
}

pub struct Response {
    res: ReqwestResponse,
}

impl Response {
    pub async fn deserialize<T: serde::de::DeserializeOwned>(&self) -> Result<T, Error> {
        self.res.json().await.map_err(Error::from)
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
        NetworkAgent { http_client: ReqwestClient::default() }
    }

    pub fn start_request(&self, method: HTTPMethod, url: &str) -> Request {
        let req = self.http_client.request(method.into(), url);
        Request::new(req)
    }

    pub async fn send_request(&self, request: Request) -> Result<Response, Error> {
        let res = self.http_client.execute(ReqwestRequest::from(request)).await?;
        Ok(Response::from(res))
    }
}
