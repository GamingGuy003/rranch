use serde_derive::Serialize;
use serde_json::Value;

#[derive(Serialize, Debug)]
pub struct Request {
    pub command: String,
    pub payload: Option<Value>,
}

impl Request {
    pub fn new(command: &str, payload: Option<Value>) -> Self {
        Self {
            command: command.to_owned(),
            payload,
        }
    }
}
