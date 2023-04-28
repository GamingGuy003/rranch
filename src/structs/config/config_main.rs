use std::{path::Path, process::exit};

use log::error;
use serde_derive::{Deserialize, Serialize};

use super::{config_client::Client, config_master::Master, config_templates::Templates};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    master: Option<Master>,
    client: Option<Client>,
    templates: Option<Templates>,
}

impl Config {
    pub fn get_master(&self) -> Master {
        self.master.clone().unwrap_or_default()
    }

    pub fn get_client(&self) -> Client {
        self.client.clone().unwrap_or_default()
    }

    pub fn get_templates(&self) -> Templates {
        self.templates.clone().unwrap_or_default()
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

        Ok(Self {
            master: Some(Master::new(
                Some(config.get_master().get_addr()),
                Some(config.get_master().get_port()),
                Some(config.get_master().get_authkey()),
                Some(config.get_master().get_fetch_url()),
            )),
            client: Some(Client::new(
                Some(config.get_client().get_name()),
                Some(config.get_client().get_type()),
                Some(config.get_client().get_loglevel()),
                Some(config.get_client().get_editor()),
                Some(protver),
            )),
            templates: Some(config.get_templates()),
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            master: Some(Master::default()),
            client: Some(Client::default()),
            templates: Some(Templates::default()),
        }
    }
}
