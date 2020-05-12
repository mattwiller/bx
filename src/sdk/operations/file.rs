use crate::sdk::models::file::File;
use crate::sdk::{Client, SDKError};
use serde::Serialize;
use std::path::Path;
use tokio::io::AsyncWriteExt;

pub struct FileOperation<'a> {
    id: &'a str,
    client: &'a mut Client,
}

impl<'a> FileOperation<'a> {
    pub fn new(id: &'a str, client: &'a mut Client) -> FileOperation<'a> {
        FileOperation { id, client }
    }

    pub async fn get(&mut self) -> Result<File, SDKError> {
        let url = format!("https://api.box.com/2.0/files/{}", self.id);
        let response = self.client.get(&url).await?;

        let file: File = response.deserialize().await?;

        Ok(file)
    }

    pub async fn delete(&mut self) -> Result<(), SDKError> {
        let url = format!("https://api.box.com/2.0/files/{}", self.id);
        self.client.delete(&url).await?;
        Ok(())
    }

    pub async fn download(&mut self, path: &Path) -> Result<(), SDKError> {
        let url = format!("https://api.box.com/2.0/files/{}/content", self.id);

        let mut response = self.client.get(&url).await?;

        let mut file = tokio::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)
            .await?;

        while let Some(bytes) = response.chunk().await? {
            file.write_all(&bytes).await?;
        }

        Ok(())
    }

    pub async fn update(&mut self, updates: FileUpdates) -> Result<File, SDKError> {
        let url = format!("https://api.box.com/2.0/files/{}", self.id);

        let response = self.client.put(&url, updates).await?;
        let file: File = response.deserialize().await?;
        Ok(file)
    }
}

#[derive(Serialize)]
pub struct FileUpdates {
    description: Option<String>,
    name: Option<String>,
    // @TODO: Implement other fields
}

impl FileUpdates {
    pub fn new() -> FileUpdates {
        FileUpdates {
            description: None,
            name: None,
        }
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
}
