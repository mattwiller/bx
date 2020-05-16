use super::file::File;
use super::folder::Folder;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum Item {
    File(Box<File>),
    Folder(Box<Folder>),
}
