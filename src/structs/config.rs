use serde_derive::{Deserialize, Serialize};
use std::process::exit;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    master: Option<Master>,
    client: Option<Client>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Master {
    addr: Option<String>,
    port: Option<i32>,
    authkey: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Client {
    name: Option<String>,
    r#type: Option<String>,
    loglevel: Option<String>,
    editor: Option<String>,
}

impl Config {
    pub fn new_from_cfg(filename: &str) -> Self {
        let file = match std::fs::read_to_string(filename) {
            Ok(file) => file,
            Err(err) => {
                println!("Error reading config file: {}", err);
                println!("Falling back to default config...");
                println!("To configure the client, create and edit rranch.toml at {}, according to the instructions on the github repo or specify an alternate config using the -cf flag.", filename);
                exit(-1)
            }
        };

        let config = match toml::from_str(file.as_str()) {
            Ok(config) => config,
            Err(err) => {
                println!(
                    "Failed to parse toml from config file {}: {}",
                    filename, err
                );
                exit(-1)
            }
        };
        config
    }

    pub fn get_master(&self) -> Master {
        self.master.clone().unwrap_or_else(Master::empty)
    }

    pub fn get_client(&self) -> Client {
        self.client.clone().unwrap_or_else(Client::empty)
    }
}

impl Master {
    pub fn empty() -> Self {
        Self {
            addr: None,
            port: None,
            authkey: None,
        }
    }

    pub fn get_addr(&self) -> String {
        self.addr.clone().unwrap_or_else(|| "localhost".to_owned())
    }

    pub fn get_port(&self) -> i32 {
        self.port.unwrap_or(27015)
    }

    pub fn get_authkey(&self) -> String {
        self.authkey.clone().unwrap_or_default()
    }
}

impl Client {
    pub fn empty() -> Self {
        Self {
            name: None,
            r#type: None,
            loglevel: None,
            editor: None,
        }
    }

    pub fn get_name(&self) -> String {
        self.name
            .clone()
            .unwrap_or_else(|| "a-rranch-client".to_owned())
    }

    pub fn get_type(&self) -> String {
        self.r#type
            .clone()
            .unwrap_or_else(|| "CONTROLLER".to_owned())
    }

    pub fn get_loglevel(&self) -> String {
        self.loglevel.clone().unwrap_or_else(|| "INFO".to_owned())
    }

    pub fn get_editor(&self) -> String {
        self.editor.clone().unwrap_or_else(|| "vim".to_owned())
    }
}
