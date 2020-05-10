use reqwest::{Client as ReqwestClient, Request as ReqwestRequest, Response as ReqwestResponse, Method as ReqwestMethod, Body as ReqwestBody};
use reqwest::header::HeaderMap as ReqwestHeaders;
use serde_json::Value;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use bytes::Bytes;

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
}

impl From<Body> for ReqwestBody {
    fn from(body: Body) -> ReqwestBody {
        match body {
            Body::Empty => ReqwestBody::from(""),
            Body::JSON(json) => ReqwestBody::from(json.to_string()),
        }
    }
}

pub struct Request {
    url: String,
    method: HTTPMethod,
    body: Body,
    headers: HashMap<String, String>,
}

impl Request {
    pub fn new(method: HTTPMethod, url: String) -> Request {
        Request {
            url,
            method,
            body: Body::Empty,
            headers: HashMap::with_capacity(3),
        }
    }

    pub fn set_body(&mut self, body: Body) {
        self.body = body;
    }

    pub fn add_header(&mut self, name: &str, value: &str) {
        self.headers.insert(name.to_owned(), value.to_owned());
    }
}

impl TryFrom<Request> for ReqwestRequest {
    fn try_from(req: Request) -> Result<ReqwestRequest, NetworkError> {
        let reqwest = ReqwestRequest::new(req.method.into(), req.url.parse()?);
        *reqwest.body_mut() = match req.body {
            Body::Empty => None,
            Body::JSON(json) => Some(ReqwestBody::from(json.to_string())),
        }; 
        *reqwest.headers_mut() = {
            let headers = ReqwestHeaders::new();
            for (k, v) in req.headers.iter() {
                headers.append(k, v);
            }
            headers
        };
        Ok(reqwest)
    }
}

pub struct Response {
    status: u16,
    body: Bytes,
    headers: HashMap<String, String>,
}

impl TryFrom<ReqwestResponse> for Response {
    fn try_from(res: ReqwestResponse) -> Result<Response, NetworkError> {
        let response = Response {
            status: res.status().as_u16(),
            body: res.bytes().await?,
            headers: {
                let h = res.headers();
                let headers = HashMap::with_capacity(h.keys_len());
                for (k, v) in h {
                    headers.insert(k.to_string(), String::from(v.to_str()?));
                }
                headers
            }
        };

        Ok(response);
    }
}


pub struct NetworkAgent {
    httpClient: ReqwestClient,
}

impl NetworkAgent {
    pub async fn send_request(&self, request: Request) -> Result<Response, NetworkError> {
        let response = self.httpClient.execute(request.try_into()?).await;
        return response.into();
    }
}

pub enum NetworkError {
    InvalidURL,
    InvalidHeader,
}

impl From<url::ParseError> for NetworkError {
    fn from(e: url::ParseError) -> NetworkError {
        NetworkError::InvalidURL
    }
}

impl From<http::header::ToStrError> for NetworkError {
    fn from(e: http::header::ToStrError) -> NetworkError {
        NetworkError::InvalidHeader
    }
}

