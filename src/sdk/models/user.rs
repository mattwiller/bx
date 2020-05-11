use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    r#type: String,
    id: String,
    name: String,
    login: String,
}
