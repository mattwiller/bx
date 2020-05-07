use super::file::File;
use super::folder::Folder;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Item {
    File(Box<File>),
    Folder(Box<Folder>),
}
