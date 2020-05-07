use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Collection<T> {
    total_count: u64,
    entries: Vec<T>,
}
