use super::collection::Collection;
use super::folder::Folder;
use super::shared_link::SharedLink;
use super::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ItemStatus {
    Active,
    Trashed,
    Deleted,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File {
    etag: Option<String>,
    id: String,
    name: Option<String>,
    sequence_id: Option<String>,
    allowed_invitee_roles: Option<Vec<String>>,
    content_created_at: Option<String>,
    content_modified_at: Option<String>,
    created_at: Option<String>,
    created_by: Option<Box<User>>,
    description: Option<String>,
    file_version: Option<Box<FileVersion>>,
    has_collaborations: Option<bool>,
    is_externally_owned: Option<bool>,
    item_status: Option<Box<ItemStatus>>,
    modified_at: Option<String>,
    modified_by: Option<Box<User>>,
    owned_by: Option<Box<User>>,
    parent: Option<Box<Folder>>,
    path_collection: Option<Collection<Folder>>,
    purged_at: Option<String>,
    sha1: Option<String>,
    shared_link: Option<SharedLink>,
    size: Option<u64>,
    tags: Option<Vec<String>>,
    trashed_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileVersion {
    id: String,
    sha1: String,
}
