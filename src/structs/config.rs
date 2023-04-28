use std::{path::Path, process::exit};

use log::error;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub master: Option<Master>,
    pub client: Option<Client>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Master {
    pub addr: Option<String>,
    pub port: Option<i32>,
    pub authkey: Option<String>,
    pub fetch_url: Option<String>,
}

impl Master {
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Client {
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub loglevel: Option<String>,
    pub editor: Option<String>,
    #[serde(skip_serializing)]
    pub protver: Option<u16>,
}

impl Client {
    pub fn get_name(&self) -> String {
        self.name.clone().unwrap_or("a-rranch-client".to_owned())
    }

    pub fn get_type(&self) -> String {
        self.r#type.clone().unwrap_or("CONTROLLER".to_owned())
    }

    pub fn get_loglevel(&self) -> String {
        self.loglevel.clone().unwrap_or("INFO".to_owned())
    }

    pub fn get_editor(&self) -> String {
        self.editor.clone().unwrap_or("vim".to_owned())
    }

    pub fn get_protver(&self) -> u16 {
        self.protver.unwrap_or_default()
    }
}

impl Default for Client {
    fn default() -> Self {
        Self {
            name: Some("a-rranch-client".to_owned()),
            r#type: Some("CONTROLLER".to_owned()),
            loglevel: Some("INFO".to_owned()),
            editor: Some("vim".to_owned()),
            protver: None,
        }
    }
}

impl Config {
    pub fn get_master(&self) -> Master {
        self.master.clone().unwrap_or_default()
    }

    pub fn get_client(&self) -> Client {
        self.client.clone().unwrap_or_default()
    }

    pub fn new_from_cfg(filename: &str, protver: u16) -> Result<Self, std::io::Error> {
        let path = Path::new(filename);
        if !path.exists() {
            println!("Creating default config at {filename}. For more information visit https://github.com/GamingGuy003/rranch");
            std::fs::create_dir_all(path.parent().unwrap_or(Path::new("")))?;
            std::fs::write(filename, toml::to_string(&Config::default()).unwrap())?;
        }
        let file = std::fs::read_to_string(filename)?;

        let config: Config = match toml::from_str(file.as_str()) {
            Ok(config) => config,
            Err(err) => {
                error!("Failed to parse toml from config file {}: {}", filename, err);
                exit(-1)
            }
        };

        let name = config.get_client().get_name();
        let r#type = config.get_client().get_type();
        let loglevel = config.get_client().get_loglevel();
        let editor = config.get_client().get_editor();
        let protver = protver;
        let addr = config.get_master().get_addr();
        let port = config.get_master().get_port();
        let authkey = config.get_master().get_authkey();
        let fetch_url = config.get_master().get_fetch_url();

        Ok(Self {
            master: Some(Master {
                addr: Some(addr),
                port: Some(port),
                authkey: Some(authkey),
                fetch_url: Some(fetch_url),
            }),
            client: Some(Client {
                name: Some(name),
                r#type: Some(r#type),
                loglevel: Some(loglevel),
                editor: Some(editor),
                protver: Some(protver),
            }),
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            master: Some(Master::default()),
            client: Some(Client::default()),
        }
    }
}
