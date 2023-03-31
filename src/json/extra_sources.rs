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

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtraSourceSubmit {
    pub description: String,
    pub filename: String,
    pub filelen: String,
}

impl ExtraSourceSubmit {
    pub fn new(description: &str, filename: &str, filelen: &str) -> Self {
        Self {
            description: description.to_owned(),
            filename: filename.to_owned(),
            filelen: filelen.to_owned(),
        }
    }
}
