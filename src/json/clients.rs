use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Clients {
    pub controllers: Vec<String>,
    pub buildbots: Vec<String>,
}
