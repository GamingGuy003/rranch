use std::path::Path;

use serde_derive::Serialize;

#[derive(Serialize)]
pub struct ExtraSourceSubmit {
    pub filename: String,
    pub filedescription: String,
    pub filelength: usize,
}

impl ExtraSourceSubmit {
    pub fn new(path: &str, filedescription: &str) -> Result<Self, std::io::Error> {
        let path = Path::new(path);
        if !path.is_file() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                String::from("Path does not lead to a file"),
            ));
        }
        let file = std::fs::read(path)?;
        Ok(Self {
            filename: path
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .to_string(),
            filedescription: filedescription.to_owned(),
            filelength: file.len(),
        })
    }
}
