use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraSource {
    pub id: String,
    pub filename: String,
    pub description: String,
}
