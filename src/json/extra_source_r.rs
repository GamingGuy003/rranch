use std::fmt::Display;

use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct ExtraSourceReceive {
    pub id: String,
    pub filename: String,
    pub description: String,
}

impl Display for ExtraSourceReceive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!("{:<40} {:<35} {}", self.id, self.filename, self.description)
        )
    }
}
