use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Dependers {
    pub releasebuild: Vec<String>,
    pub crossbuild: Vec<String>,
}
