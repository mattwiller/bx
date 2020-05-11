use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Folder {
    r#type: String,
    id: String,
    etag: Option<String>,
    name: Option<String>,
    sequence_id: Option<String>,
}
