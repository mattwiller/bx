use std::fmt;

#[derive(Debug)]
pub enum Error {
    Network(reqwest::Error),
    Auth,
    Deserialize(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Auth => write!(f, "Authentication error"),
            Error::Network(e) => write!(f, "Network error: {}", e),
            Error::Deserialize(e) => write!(f, "Deserialiazation error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Auth => None,
            Error::Network(e) => Some(e),
            Error::Deserialize(e) => Some(e),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::Network(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::Deserialize(e)
    }
}
