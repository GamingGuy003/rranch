use std::process::exit;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub master: Option<Master>,
    pub client: Option<Client>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Master {
    pub addr: Option<String>,
    pub port: Option<i32>,
    pub authkey: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Client {
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub loglevel: Option<String>,
    pub editor: Option<String>,
}

impl Config {
    pub fn new_from_cfg(filename: &str) -> Self {
        let file = match std::fs::read_to_string(filename) {
            Ok(file) => {
                file
            }
            Err(err) => {
                println!("Error reading config file: {}", err);
                println!("Falling back to default config...");
                println!("To configure the client, create and edit rranch.toml at {}, according to the instructions on the github repo or specify an alternate config using the -cf flag.", filename);
                "".to_owned()
            }
        };

        let config: Config = match toml::from_str(file.as_str()) {
            Ok(config) => config,
            Err(err) => {
                println!(
                    "Failed to parse toml from config file {}: {}",
                    filename, err
                );
                exit(-1)
            }
        };

        let mut name = "a-rranch-client".to_owned();
        let mut r#type = "CONTROLLER".to_owned();
        let mut loglevel = "NONE".to_owned();
        let mut editor = "vim".to_owned();
        let mut addr = "localhost".to_owned();
        let mut port = 27015;
        let mut authkey = "".to_owned();

        if config.client.as_ref().is_some()
            && config
                .client
                .as_ref()
                .unwrap_or(&Client::empty())
                .name
                .is_some()
        {
            name = config
                .client
                .as_ref()
                .unwrap_or(&Client::empty())
                .name
                .as_ref()
                .unwrap_or(&"a-rranch-client".to_string())
                .to_string();
        }

        if config.client.as_ref().is_some()
            && config
                .client
                .as_ref()
                .unwrap_or(&Client::empty())
                .r#type
                .is_some()
        {
            r#type = config
                .client
                .as_ref()
                .unwrap_or(&Client::empty())
                .r#type
                .as_ref()
                .unwrap_or(&"CONTROLLER".to_string())
                .to_string();
        }

        if config.client.as_ref().is_some()
            && config
                .client
                .as_ref()
                .unwrap_or(&Client::empty())
                .loglevel
                .is_some()
        {
            loglevel = config
                .client
                .as_ref()
                .unwrap_or(&Client::empty())
                .loglevel
                .as_ref()
                .unwrap_or(&"none".to_string())
                .to_string();
        }

        if config.client.as_ref().is_some()
            && config
                .client
                .as_ref()
                .unwrap_or(&Client::empty())
                .editor
                .is_some()
        {
            editor = config
                .client
                .as_ref()
                .unwrap_or(&Client::empty())
                .editor
                .as_ref()
                .unwrap_or(&"vim".to_string())
                .to_string();
        }

        if config.master.as_ref().is_some()
            && config
                .master
                .as_ref()
                .unwrap_or(&Master::empty())
                .addr
                .is_some()
        {
            addr = config
                .master
                .as_ref()
                .unwrap_or(&Master::empty())
                .addr
                .as_ref()
                .unwrap_or(&"localhost".to_string())
                .to_string();
        }

        if config.master.as_ref().is_some()
            && config
                .master
                .as_ref()
                .unwrap_or(&Master::empty())
                .port
                .is_some()
        {
            port = *config
                .master
                .as_ref()
                .unwrap_or(&Master::empty())
                .port
                .as_ref()
                .unwrap_or(&27015);
        }

        if config.master.as_ref().is_some()
            && config
                .master
                .as_ref()
                .unwrap_or(&Master::empty())
                .authkey
                .is_some()
        {
            authkey = config
                .master
                .as_ref()
                .unwrap_or(&Master::empty())
                .authkey
                .as_ref()
                .unwrap_or(&"defautl".to_string())
                .to_string();
        }

        Self {
            master: Some(Master {
                addr: Some(addr),
                port: Some(port),
                authkey: Some(authkey),
            }),
            client: Some(Client {
                name: Some(name),
                r#type: Some(r#type),
                loglevel: Some(loglevel),
                editor: Some(editor),
            }),
        }
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
}
