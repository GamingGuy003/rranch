use std::process::exit;

use log::{debug, error, info, trace, warn};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PKGBuildJson {
    //mandatory fields
    name: String,
    version: String,
    real_version: String,
    //optional fields
    dependencies: String,
    build_dependencies: String,
    cross_dependencies: String,
    source: String,
    extra_sources: Vec<String>,
    description: String,
    build_script: Vec<String>,
}

impl PKGBuildJson {
    pub fn to_pkgbuild(&self) -> Vec<String> {
        let mut bpb = Vec::new();
        bpb.push(format!("name={}", self.name));
        bpb.push(format!("version={}", self.version));

        if self.description.len() > 0 {
            bpb.push(format!("description={}", self.description));
        }

        bpb.push(format!("real_version={}", self.real_version));

        if self.source.len() > 0 {
            bpb.push(format!("source={}", self.source));
        }

        if self.extra_sources.len() > 0 {
            let mut xsrc = "extra_sources=".to_owned();
            for src in self.extra_sources.clone() {
                xsrc = format!("{}[{}]", xsrc, src);
            }
            bpb.push(format!("extra_sources={}", xsrc));
        }

        if self.dependencies.len() > 0 {
            bpb.push(format!("dependencies={}", self.dependencies));
        }

        if self.build_dependencies.len() > 0 {
            bpb.push(format!("builddeps={}", self.build_dependencies));
        }

        if self.cross_dependencies.len() > 0 {
            bpb.push(format!("crossdeps={}", self.cross_dependencies));
        }

        if self.build_script.len() > 0 {
            bpb.push(format!(
                "build={{\n\t{}\n}}",
                self.build_script.join("\n\t")
            ));
        }

        bpb
    }

    pub fn from_pkgbuild(bpb: &str) -> Self {
        trace!("Reading and parsing pkgbuild file...");
        let file = match std::fs::read_to_string(bpb) {
            Ok(file) => {
                trace!("Successfully read file {}", bpb);
                file
            }
            Err(err) => {
                error!("Error reading pkgbuild file: {}", err);
                exit(-1)
            }
        };

        let lines = file.lines();
        let mut build = false;
        let mut i = 1;
        let mut ret = Self {
            name: "".to_owned(),
            version: "".to_owned(),
            real_version: "".to_owned(),
            dependencies: "".to_owned(),
            build_dependencies: "".to_owned(),
            cross_dependencies: "".to_owned(),
            source: "".to_owned(),
            extra_sources: Vec::new(),
            description: "".to_owned(),
            build_script: Vec::new(),
        };

        for line in lines {
            if build {
                if line.starts_with("}") {
                    build = false;
                    continue;
                }
                ret.build_script.push(line.replace("\t", ""));
            } else {
                if line.len() == 0 {
                    continue;
                }
                let split: Vec<&str> = line.split("=").collect();
                if split.len() != 2 {
                    error!("Invalid syntax at line {}", i);
                    exit(-1)
                }
                match split[0] {
                    "name" => ret.name = split[1].to_owned(),
                    "version" => ret.version = split[1].to_owned(),
                    "real_version" => ret.real_version = split[1].to_owned(),
                    "description" => ret.description = split[1].to_owned(),
                    "source" => ret.source = split[1].to_owned(),
                    "extra_sources" => {
                        ret.extra_sources = split[1]
                            .split("[")
                            .collect::<Vec<&str>>()
                            .into_iter()
                            .filter(|part| !part.is_empty())
                            .map(|part| part.trim_end_matches(']'))
                            .map(|part| part.to_string())
                            .collect()
                    }
                    "dependencies" => ret.dependencies = split[1].to_owned(),
                    "builddeps" => ret.build_dependencies = split[1].to_owned(),
                    "crossdeps" => ret.cross_dependencies = split[1].to_owned(),
                    "build" => build = true,
                    _ => warn!("Found invalid key at line {}", i),
                }
            }
            i += 1;
        }

        ret
    }

    pub fn new_template() -> Self {
        let mut ret = Self {
            name: "template".to_owned(),
            version: "0".to_owned(),
            real_version: "0".to_owned(),
            dependencies: "[]".to_owned(),
            build_dependencies: "[]".to_owned(),
            cross_dependencies: "[]".to_owned(),
            source: " ".to_owned(),
            extra_sources: Vec::new(),
            description: "".to_owned(),
            build_script: Vec::new(),
        };
        ret.extra_sources.push(String::new());
        ret.build_script.push("".to_owned());
        ret
    }

    pub fn create_workdir(&self) {
        trace!("Creating workdir for {}", self.name);
        let path = self.name.as_str();
        if let Ok(_) = std::fs::metadata(path) {
            warn!("Build dir exists, overwriting...");
            match std::fs::remove_dir_all(path) {
                Ok(_) => debug!("Removed old dir"),
                Err(err) => {
                    error!("Error removing directory: {}", err);
                    exit(-1)
                }
            }
        } else {
            info!("Creating build workdir...");
        }

        match std::fs::create_dir(path) {
            Ok(_) => debug!("Successfully created directory {}", path),
            Err(err) => {
                error!("Error creating directory: {}", err);
                exit(-1)
            }
        }

        match std::fs::write(
            format!("{}/package.bpb", path),
            self.to_pkgbuild().join("\n"),
        ) {
            Ok(_) => debug!("Successfully wrote pkgbuild file"),
            Err(err) => {
                error!("Error creating pkgbuild file: {}", err);
                exit(-1)
            }
        }
    }
}
