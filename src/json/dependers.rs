use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Dependers {
    pub crossbuild: Vec<String>,
    pub releasebuild: Vec<String>,
}
