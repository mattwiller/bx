use crate::sdk::models::{Collection, Folder, Item};
use crate::sdk::{Client, Response, SDKError};

pub struct FolderOperation<'a> {
    id: &'a str,
    client: &'a mut Client,
}

impl<'a> FolderOperation<'a> {
    pub fn new(id: &'a str, client: &'a mut Client) -> FolderOperation<'a> {
        FolderOperation { id, client }
    }

    pub async fn get(&mut self) -> Result<Folder, SDKError> {
        let url = format!("https://api.box.com/2.0/folders/{}", self.id);
        let response = self.client.get(&url).await?;

        let folder: Folder = response.deserialize().await?;

        Ok(folder)
    }

    pub async fn get_items(&mut self, limit: u32) -> Result<Collection<Item>, SDKError> {
        let url = format!(
            "https://api.box.com/2.0/folders/{}/items?limit={}",
            self.id, limit
        );

        let response: Response = self.client.get(&url).await?;
        let items: Collection<Item> = response.deserialize().await?;

        Ok(items)
    }
}
