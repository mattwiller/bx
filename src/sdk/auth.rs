use super::SDKError;
use async_trait::async_trait;

pub struct AccessToken(String);

impl AccessToken {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<String> for AccessToken {
    fn from(t: String) -> AccessToken {
        AccessToken(t)
    }
}

#[async_trait]
pub trait Auth {
    async fn token(&mut self) -> Result<&AccessToken, SDKError>;
}

pub struct SingleTokenAuth {
    token: AccessToken,
}

impl SingleTokenAuth {
    pub fn new(token: String) -> SingleTokenAuth {
        SingleTokenAuth {
            token: AccessToken::from(token),
        }
    }
}

#[async_trait]
impl Auth for SingleTokenAuth {
    async fn token(&mut self) -> Result<&AccessToken, SDKError> {
        Ok(&self.token)
    }
}
