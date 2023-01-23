use std::process::exit;

use log::{debug, error, info, trace, warn};
use serde_derive::{Deserialize, Serialize};

use crate::util::util::get_choice;

#[derive(Deserialize, Serialize)]
pub struct PKGBuildJson {
    //mandatory fields
    name: String,
    version: String,
    real_version: String,
    //optional fields
    source: String,
    dependencies: Vec<String>,
    build_dependencies: Vec<String>,
    cross_dependencies: Vec<String>,
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
            bpb.push(xsrc);
        }

        if self.dependencies.len() > 0 {
            let mut dependencies = "dependencies=".to_owned();
            for dependency in self.dependencies.clone() {
                dependencies = format!("{}[{}]", dependencies, dependency);
            }
            bpb.push(dependencies);
        }

        if self.build_dependencies.len() > 0 {
            let mut build_dependencies = "builddeps=".to_owned();
            for build_dependency in self.build_dependencies.clone() {
                build_dependencies = format!("{}[{}]", build_dependencies, build_dependency);
            }
            bpb.push(build_dependencies);
        }

        if self.cross_dependencies.len() > 0 {
            let mut cross_dependencies = "crossdeps=".to_owned();
            for cross_dependency in self.build_dependencies.clone() {
                cross_dependencies = format!("{}[{}]", cross_dependencies, cross_dependency);
            }
            bpb.push(cross_dependencies);
        }

        if self.build_script.len() > 0 {
            let mut build = "build={".to_owned();
            for build_line in self.build_script.clone() {
                if build_line.starts_with("\t") {
                    build = format!("{}\n{}", build, build_line);
                } else {
                    build = format!("{}\n\t{}", build, build_line);
                }
            }
            build = format!("{}\n}}", build);
            bpb.push(build);
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
            dependencies: Vec::new(),
            build_dependencies: Vec::new(),
            cross_dependencies: Vec::new(),
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
                ret.build_script.push(line.to_owned());
            } else {
                if line.len() == 0 {
                    continue;
                }
                let split: Vec<&str> = line.splitn(2, "=").collect();
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
                            .map(|part| part.trim_end_matches(']'))
                            .map(|part| part.to_string())
                            .filter(|part| !part.is_empty())
                            .collect()
                    }
                    "dependencies" => {
                        ret.dependencies = split[1]
                            .split("[")
                            .collect::<Vec<&str>>()
                            .into_iter()
                            .map(|part| part.trim_end_matches(']'))
                            .map(|part| part.to_string())
                            .filter(|part| !part.is_empty())
                            .collect()
                    }
                    "builddeps" => {
                        ret.build_dependencies = split[1]
                            .split("[")
                            .collect::<Vec<&str>>()
                            .into_iter()
                            .map(|part| part.trim_end_matches(']'))
                            .map(|part| part.to_string())
                            .filter(|part| !part.is_empty())
                            .collect()
                    }
                    "crossdeps" => {
                        ret.cross_dependencies = split[1]
                            .split("[")
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
        println!(
            "Sending to server:\n{}",
            serde_json::to_string_pretty(&ret).unwrap()
        );
        ret
    }

    pub fn new_template() -> Self {
        let mut ret = Self {
            name: "template".to_owned(),
            version: "0".to_owned(),
            real_version: "0".to_owned(),
            description: " ".to_owned(),
            dependencies: Vec::new(),
            build_dependencies: Vec::new(),
            cross_dependencies: Vec::new(),
            source: " ".to_owned(),
            extra_sources: Vec::new(),
            build_script: Vec::new(),
        };
        ret.dependencies.push(String::new());
        ret.build_dependencies.push(String::new());
        ret.cross_dependencies.push(String::new());
        ret.extra_sources.push(String::new());
        ret.build_script
            .push("cd $PKG_NAME-$PKG_VERSION".to_owned());
        ret.build_script
            .push("make DESTDIR=$PKG_INSTALL_DIR install".to_owned());
        ret
    }

    pub fn create_workdir(&self) {
        trace!("Creating workdir for {}", self.name);
        let path = self.name.as_str();
        if let Ok(_) = std::fs::metadata(path) {
            if get_choice("Build dir already exists. Do you want to overwrite it?") {
                warn!("Overwriting existing builddir...");
                match std::fs::remove_dir_all(path) {
                    Ok(_) => debug!("Removed old dir"),
                    Err(err) => {
                        error!("Error removing directory: {}", err);
                        exit(-1)
                    }
                }
            } else {
                error!("Abortet due to user choice");
                return;
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

    pub fn get_dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }

    pub fn get_build_dependencies(&self) -> Vec<String> {
        self.build_dependencies.clone()
    }

    pub fn get_cross_dependencies(&self) -> Vec<String> {
        self.cross_dependencies.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
