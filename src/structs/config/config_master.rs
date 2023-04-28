use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Master {
    addr: Option<String>,
    port: Option<i32>,
    authkey: Option<String>,
    fetch_url: Option<String>,
}

impl Master {
    pub fn new(addr: Option<String>, port: Option<i32>, authkey: Option<String>, fetch_url: Option<String>) -> Self {
        Self { addr, port, authkey, fetch_url }
    }

    pub fn get_addr(&self) -> String {
        self.addr.clone().unwrap_or("localhost".to_owned())
    }

    pub fn get_port(&self) -> i32 {
        self.port.unwrap_or(27015)
    }

    pub fn get_authkey(&self) -> String {
        self.authkey.clone().unwrap_or_default()
    }

    pub fn get_fetch_url(&self) -> String {
        self.fetch_url.clone().unwrap_or_default()
    }
}

impl Default for Master {
    fn default() -> Self {
        Self {
            addr: Some("localhost".to_owned()),
            port: Some(27015),
            authkey: Some("".to_owned()),
            fetch_url: Some("https://localhost".to_owned()),
        }
    }
}
