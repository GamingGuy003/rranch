use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct Templates {
    templates: Option<HashMap<String, Vec<String>>>,
}

impl Templates {
    pub fn get_templates(&self) -> HashMap<String, Vec<String>> {
        self.templates.clone().unwrap_or_default()
    }
}

impl Default for Templates {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(
            String::from("make"),
            vec![
                String::from("\tcd $PKG_NAME-$PKG_VERSION"),
                String::from("\t"),
                String::from("\tmake -j$(nproc)"),
                String::from("\tmake DESTDIR=$PKG_INSTALL_DIR install"),
            ],
        );
        map.insert(
            String::from("ninja"),
            vec![
                String::from("\tcd $PKG_NAME-$PKG_VERSION"),
                String::from("\t"),
                String::from("\tmkdir build"),
                String::from("cd build"),
                String::from("\tDESTDIR=$PKG_INSTALL_DIR ninja install"),
            ],
        );
        Self { templates: Some(map) }
    }
}
