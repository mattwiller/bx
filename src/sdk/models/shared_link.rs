use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SharedLinkAccess {
    Open,
    Company,
    Collaborators,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SharedLinkPermission {
    CanDownload,
    CanPreview,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SharedLinkPermissions {
    can_download: bool,
    can_preview: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SharedLink {
    access: SharedLinkAccess,
    download_count: u64,
    effective_access: SharedLinkAccess,
    effective_permission: SharedLinkPermission,
    is_password_enabled: bool,
    permissions: SharedLinkPermissions,
    preview_count: u64,
    unshared_at: Option<String>,
    url: String,
    vanity_url: Option<String>,
}
