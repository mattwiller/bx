use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Collection<T> {
    pub total_count: u64,
    pub entries: Vec<T>,
}
