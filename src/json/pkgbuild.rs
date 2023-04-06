use serde_derive::Deserialize;

use crate::util::funcs::get_yn;

#[derive(Debug, Deserialize)]
pub struct PackageBuild {
    pub name: String,
    pub description: String,
    pub version: String,
    pub real_version: String,
    pub source: String,
    pub dependencies: Vec<String>,
    pub build_dependencies: Vec<String>,
    pub cross_dependencies: Vec<String>,
    pub extra_sources: Vec<String>,
    pub build_script: Vec<String>,
}

impl PackageBuild {
    pub fn to_vec(&self) -> Result<Vec<String>, std::io::Error> {
        let mut lines = Vec::new();

        lines.push(format!("name={}", self.name));
        lines.push(format!("version={}", self.version));
        lines.push(format!("description={}", self.description));
        lines.push(format!("version={}", self.version));
        lines.push(format!("real_version={}", self.real_version));
        lines.push(format!("source={}", self.source));

        let mut tmp = String::from("dependencies=");
        self.dependencies
            .iter()
            .for_each(|dependency| tmp = format!("{tmp}[{dependency}]"));
        lines.push(tmp);

        let mut tmp = String::from("builddeps=");
        self.build_dependencies
            .iter()
            .for_each(|build_dependency| tmp = format!("{tmp}[{build_dependency}]"));
        lines.push(tmp);

        let mut tmp = String::from("crossdeps=");
        self.cross_dependencies
            .iter()
            .for_each(|cross_dependency| tmp = format!("{tmp}[{cross_dependency}]"));
        lines.push(tmp);

        let mut tmp = String::from("extra_sources=");
        self.extra_sources
            .iter()
            .for_each(|extra_source| tmp = format!("{tmp}[{extra_source}]"));
        lines.push(tmp);

        let mut tmp = Vec::new();
        tmp.push(String::from("build={"));
        self.build_script
            .iter()
            .for_each(|build_line| tmp.push(build_line.to_owned()));
        tmp.push(String::from("}"));
        lines.push(tmp.join("\n"));

        Ok(lines)
    }

    pub fn create_workdir(&mut self) -> Result<(), std::io::Error> {
        let path = self.name.as_str();

        if std::fs::metadata(path).is_ok() {
            if get_yn(
                "Package build exists locally, do you want to overwrite it?",
                false,
            )? {
                std::fs::remove_dir_all(path)?;
            } else {
                return Ok(());
            }
        }

        std::fs::create_dir(path)?;
        std::fs::write(format!("{path}/package.bpb"), self.to_vec()?.join("\n"))
    }
}
