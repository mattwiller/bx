use crate::sdk::models::user::User;
use crate::sdk::{Client, SDKError};

pub struct UserOperation<'a> {
    id: &'a str,
    client: &'a mut Client,
}

impl<'a> UserOperation<'a> {
    pub fn new(id: &'a str, client: &'a mut Client) -> UserOperation<'a> {
        UserOperation { id, client }
    }

    pub async fn get(&mut self) -> Result<User, SDKError> {
        let url = format!("https://api.box.com/2.0/users/{}", self.id);
        let response = self.client.get(&url).await?;

        let user: User = response.deserialize().await?;

        Ok(user)
    }
}
