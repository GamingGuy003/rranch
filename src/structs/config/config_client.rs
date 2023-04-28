use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Client {
    name: Option<String>,
    r#type: Option<String>,
    loglevel: Option<String>,
    editor: Option<String>,
    #[serde(skip_serializing)]
    protver: Option<u16>,
}

impl Client {
    pub fn new(name: Option<String>, r#type: Option<String>, loglevel: Option<String>, editor: Option<String>, protver: Option<u16>) -> Self {
        Self {
            name,
            r#type,
            loglevel,
            editor,
            protver,
        }
    }

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
