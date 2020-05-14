use super::Response;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SDKError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("authentication error")]
    Auth,
    #[error("deserialization error: {0}")]
    Deserialize(#[from] serde_json::Error),
    #[error("invalid url: {0}")]
    InvalidURL(#[from] url::ParseError),
    #[error("invalid header: {0}")]
    InvalidHeader(#[from] http::Error),
    #[error("error reading file: {0}")]
    FileIO(#[from] tokio::io::Error),
    #[error("box api error: {}", .response.status())]
    APIError { response: Response },
}
