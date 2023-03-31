use std::fmt::Display;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraSource {
    pub id: String,
    pub filename: String,
    pub description: String,
}

impl Display for ExtraSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "{:<40} {:<30}\t{}",
                self.id, self.filename, self.description
            )
        )
    }
}
