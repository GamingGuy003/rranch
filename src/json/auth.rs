use serde_derive::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct AuthRequest {
    pub machine_identifier: String,
    pub machine_type: String,
    pub machine_authkey: String,
    pub machine_version: i32,
}

impl AuthRequest {
    pub fn new(
        machine_identifier: &str,
        machine_type: &str,
        machine_authkey: &str,
        machine_version: i32,
    ) -> Self {
        Self {
            machine_identifier: machine_identifier.to_owned(),
            machine_type: machine_type.to_owned(),
            machine_authkey: machine_authkey.to_owned(),
            machine_version,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub auth_status: String,
    pub logon_message: String,
}
