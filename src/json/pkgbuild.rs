use log::{trace, warn};
use serde_derive::{Deserialize, Serialize};

use crate::util::funcs::get_yn;

#[derive(Debug, Deserialize, Serialize, Default)]
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
    pub fn new() -> Self {
        Self {
            name: Default::default(),
            description: Default::default(),
            version: Default::default(),
            real_version: String::from("0"),
            source: Default::default(),
            dependencies: Default::default(),
            build_dependencies: Default::default(),
            cross_dependencies: Default::default(),
            extra_sources: Default::default(),
            build_script: Default::default(),
        }
    }
    pub fn to_vec(&self) -> Result<Vec<String>, std::io::Error> {
        let mut lines = Vec::new();

        lines.push(format!("name={}", self.name));
        lines.push(format!("version={}", self.version));
        lines.push(format!("description={}", self.description));
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
            if get_yn("Package build exists locally, do you want to overwrite it?", false)? {
                std::fs::remove_dir_all(path)?;
            } else {
                trace!("Did not write packagebuild");
                return Ok(());
            }
        }

        trace!("Writing packagebuild");
        std::fs::create_dir(path)?;
        std::fs::write(format!("{path}/package.bpb"), self.to_vec()?.join("\n"))
    }

    pub fn from_str(r#str: &str) -> Result<Self, std::io::Error> {
        let mut out = Self::default();
        let mut i = 0;
        let mut build = false;
        let lines = r#str.lines();
        for line in lines {
            if build {
                if line.starts_with('}') {
                    build = false;
                    continue;
                }
                out.build_script.push(line.to_owned());
            } else {
                if line.is_empty() {
                    continue;
                }
                let split: Vec<&str> = line.splitn(2, '=').collect();
                if split.len() != 2 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid syntax at line {}", i),
                    ));
                }
                match split[0] {
                    "name" => out.name = split[1].to_owned(),
                    "version" => out.version = split[1].to_owned(),
                    "real_version" => out.real_version = split[1].to_owned(),
                    "description" => out.description = split[1].to_owned(),
                    "source" => out.source = split[1].to_owned(),
                    "extra_sources" => {
                        out.extra_sources = split[1]
                            .split('[')
                            .collect::<Vec<&str>>()
                            .into_iter()
                            .map(|part| part.trim_end_matches(']'))
                            .map(|part| part.to_string())
                            .filter(|part| !part.is_empty())
                            .collect()
                    }
                    "dependencies" => {
                        out.dependencies = split[1]
                            .split('[')
                            .collect::<Vec<&str>>()
                            .into_iter()
                            .map(|part| part.trim_end_matches(']'))
                            .map(|part| part.to_string())
                            .filter(|part| !part.is_empty())
                            .collect()
                    }
                    "builddeps" => {
                        out.build_dependencies = split[1]
                            .split('[')
                            .collect::<Vec<&str>>()
                            .into_iter()
                            .map(|part| part.trim_end_matches(']'))
                            .map(|part| part.to_string())
                            .filter(|part| !part.is_empty())
                            .collect()
                    }
                    "crossdeps" => {
                        out.cross_dependencies = split[1]
                            .split('[')
                            .collect::<Vec<&str>>()
                            .into_iter()
                            .map(|part| part.trim_end_matches(']'))
                            .map(|part| part.to_string())
                            .filter(|part| !part.is_empty())
                            .collect()
                    }
                    "build" => build = true,
                    _ => warn!("Found invalid key at line {}", i),
                }
            }
            i += 1;
        }
        Ok(out)
    }
}
