use console::Style;
use log::{debug, error, info, warn};

use crate::{
    json::extra_sources::ExtraSource,
    structs::pkgbuild::PKGBuild,
    util::funcs::{get_choice, print_vec_cols},
};

use super::client::Client;

impl Client {
    // downloads pkgb and creates workdir
    pub fn checkout(&mut self, pkg_name: &str) -> Result<(), std::io::Error> {
        debug!("Trying to checkout {pkg_name}...");

        let resp = self.write_and_read(&format!("CHECKOUT_PACKAGE {}", pkg_name))?;

        match resp.as_str() {
            "INV_PKG_NAME" => {
                error!("Invalid package name");
                self.exit_clean(-1)?;
            }
            "INV_PKG" => {
                error!("The packagebuild is invalid");
                self.exit_clean(-1)?;
            }
            _ => {}
        }

        match serde_json::from_str::<PKGBuild>(&resp) {
            Ok(json) => json.create_workdir(),
            Err(err) => {
                error!("Failed to deserialize json: {err}");
                self.exit_clean(-1)
            }
        }
    }

    // submits package
    pub fn submit(&mut self, path: &str) -> Result<(), std::io::Error> {
        debug!("Trying to submit {path}...");

        let pkgb = PKGBuild::from_pkgbuild(path)?;
        let resp = self.write_and_read("MANAGED_PKGBUILDS")?;
        let pkgbs: Vec<String> = serde_json::from_str(resp.as_str())?;

        if pkgbs.contains(&pkgb.get_name())
            && !get_choice(
                "Packagebuild exists on remote. Do you want to overwrite it",
                false,
            )?
        {
            warn!("Aborted submit due to user choice");
            return self.exit_clean(0);
        }

        let json = serde_json::to_string(&pkgb)?;
        let resp = self.write_and_read(&format!("SUBMIT_PACKAGE {json}"))?;

        match resp.as_str() {
            "INV_PKG_BUILD" => {
                error!("Package submission rejected by server. The package build you attempted to submit is invalid.");
                Ok(())
            }
            "CMD_OK" => {
                info!("Package submission accepted by server.");
                Ok(())
            }
            other => {
                error!("Received unexpected response: {other}");
                self.exit_clean(-1)
            }
        }
    }

    // submits solution and builds it (release / cross)
    pub fn submit_sol(&mut self, rb: bool, path: &str) -> Result<(), std::io::Error> {
        debug!("Trying to submit solution: {path}; rb: {rb}...");

        let cmd = if rb {
            "SUBMIT_SOLUTION_RB"
        } else {
            "SUBMIT_SOLUTION_CB"
        };

        // parse into lines and each line into tokens separated by ';'
        let tokens = std::fs::read_to_string(path)?
            .lines()
            .map(|line| {
                line.split(';')
                    .map(|element| element.trim().to_owned())
                    .filter(|element| !element.is_empty())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();

        let resp = self.write_and_read(&format!("{cmd} {:?}", tokens))?;

        match resp.as_str() {
            "BATCH_QUEUED" => Ok(()),
            "INV_SOL" => {
                error!("Invalid solution file");
                self.exit_clean(-1)
            }
            s if s.starts_with("PKG_BUILD_MISSING") => {
                error!(
                    "Missing packagebuild on server: {}",
                    s.splitn(2, ' ').collect::<Vec<&str>>()[1]
                );
                self.exit_clean(-1)
            }
            other => {
                error!("Received unexpected response: {other}");
                self.exit_clean(-1)
            }
        }
    }

    // get list of pkgs
    pub fn get_packages(&mut self) -> Result<(), std::io::Error> {
        debug!("Trying to list pkgs...");

        let bold = Style::new().bold();
        let resp = self.write_and_read("MANAGED_PACKAGES")?;
        let mut pkgs = serde_json::from_str::<Vec<String>>(resp.as_str())?;

        println!("{}", bold.apply_to("Managed packages:"));
        if pkgs.is_empty() {
            println!("No managed packages on server");
        } else {
            pkgs.sort();
            print_vec_cols(pkgs, None, 0);
        }
        Ok(())
    }

    // get list of pkgbs
    pub fn get_packagebuilds(&mut self) -> Result<(), std::io::Error> {
        debug!("Trying to list pkgbuilds...");

        let bold = Style::new().bold();
        let resp = self.write_and_read("MANAGED_PKGBUILDS")?;
        let mut pkgbs = serde_json::from_str::<Vec<String>>(resp.as_str())?;

        println!("{}", bold.apply_to("Managed packagebuild:"));
        if pkgbs.is_empty() {
            println!("No managed packagebuilds on server");
        } else {
            pkgbs.sort();
            print_vec_cols(pkgbs, None, 0);
        }
        Ok(())
    }

    // get diff pkgbs / pkgs
    pub fn get_diff(&mut self) -> Result<(), std::io::Error> {
        debug!("Trying to list pkgbuild / pkg diff...");

        let red = Style::new().red();
        let green = Style::new().green();
        let bold = Style::new().bold();

        let pkgs = serde_json::from_str::<Vec<String>>(&self.write_and_read("MANAGED_PACKAGES")?)?;
        let mut pkgbs =
            serde_json::from_str::<Vec<String>>(&self.write_and_read("MANAGED_PKGBUILDS")?)?;
        pkgbs.sort();

        let mut diff: Vec<String> = Vec::new();
        let mut clone = pkgs.clone();
        clone.extend(pkgbs.clone());
        let max = clone
            .iter()
            .max_by_key(|value| value.len())
            .unwrap_or(&String::new())
            .len();

        pkgbs.iter().for_each(|pkgb| {
            if pkgs.contains(pkgb) {
                diff.push(green.apply_to(format!("{:<max$}", pkgb)).to_string());
            } else {
                diff.push(red.apply_to(format!("{:<max$}", pkgb)).to_string());
            }
        });

        println!("{}", bold.apply_to("Package / Packageuild diff:"));
        if diff.is_empty() {
            println!("No managed packagebuilds on server");
        } else {
            print_vec_cols(diff, Some(max as i32), 0);
        }
        Ok(())
    }

    // get dependers of pkg
    pub fn get_dependers(&mut self, pkg_name: &str) -> Result<(), std::io::Error> {
        debug!("Trying to list dependers for {pkg_name}...");

        let bold = Style::new().bold();

        let resp = self.write_and_read(&format!("GET_DEPENDERS {}", pkg_name))?;

        let dependers = match resp.as_str() {
            "INV_PKG_NAME" => {
                error!("Invalid package name!");
                return self.exit_clean(-1);
            }
            json => serde_json::from_str::<Vec<String>>(json)?,
        };

        println!("{}", bold.apply_to(format!("Dependers on {}:", pkg_name)));
        print_vec_cols(dependers, None, 0);
        Ok(())
    }

    // get dependencies of pkg
    pub fn get_dependencies(&mut self, pkg_name: &str) -> Result<(), std::io::Error> {
        debug!("Trying to list dependencies of {}...", pkg_name);

        let bold = Style::new().bold(); //title
        let red = Style::new().red(); // unbuilt dependencies without packagebuild
        let yellow = Style::new().yellow(); //unbuilt dependencies with packagebuild
        let green = Style::new().green(); // built dependencies with packagebuild

        let resp = self.write_and_read(&format!("CHECKOUT_PACKAGE {}", pkg_name))?;

        match resp.as_str() {
            "INV_PKG_NAME" => {
                error!("Invalid package name!");
                return self.exit_clean(-1);
            }
            "INV_PKG" => {
                error!("Invalid packagebuild!");
                return self.exit_clean(-1);
            }
            _ => {}
        }

        let pkgb = serde_json::from_str::<PKGBuild>(&resp)?;
        let pkgs: Vec<String> =
            serde_json::from_str(self.write_and_read("MANAGED_PACKAGES")?.as_str())?;
        let pkgbs =
            serde_json::from_str::<Vec<String>>(&self.write_and_read("MANAGED_PKGBUILDS")?)?;

        let deps = pkgb.get_dependencies();
        let maxdeps = deps
            .iter()
            .max_by_key(|dep| dep.len())
            .unwrap_or(&String::new())
            .len();
        let bdeps = pkgb.get_build_dependencies();
        let maxbdeps = bdeps
            .iter()
            .max_by_key(|bdep| bdep.len())
            .unwrap_or(&String::new())
            .len();
        let cdeps = pkgb.get_cross_dependencies();
        let maxcdeps = cdeps
            .iter()
            .max_by_key(|cdep| cdep.len())
            .unwrap_or(&String::new())
            .len();

        let mut diffdeps: Vec<String> = Vec::new();
        let mut diffbdeps: Vec<String> = Vec::new();
        let mut diffcdeps: Vec<String> = Vec::new();

        deps.iter().for_each(|dep| {
            if pkgbs.contains(&dep) {
                if pkgs.contains(&dep) {
                    diffdeps.push(format!("{}", green.apply_to(dep))); // packagebuild and binary
                } else {
                    diffdeps.push(format!("{}", yellow.apply_to(dep))); // packagebuild no binary
                }
            } else {
                diffdeps.push(format!("{}", red.apply_to(dep))); // no packagebuild no binary
            }
        });

        for dep in bdeps.clone() {
            if pkgbs.contains(&dep) {
                if pkgs.contains(&dep) {
                    diffbdeps.push(format!("{}", green.apply_to(dep))); // packagebuild and binary
                } else {
                    diffbdeps.push(format!("{}", yellow.apply_to(dep))); // packagebuild no binary
                }
            } else {
                diffbdeps.push(format!("{}", red.apply_to(dep))); // no packagebuild and no binary
            }
        }

        for dep in cdeps.clone() {
            if pkgbs.contains(&dep) {
                if pkgs.contains(&dep) {
                    diffcdeps.push(format!("{}", green.apply_to(dep))); //packagebuild and binary
                } else {
                    diffcdeps.push(format!("{}", yellow.apply_to(dep))); //packagebuild no binary
                }
            } else {
                diffcdeps.push(format!("{}", red.apply_to(dep))); //no packagebuild no binary
            }
        }

        println!(
            "{}",
            bold.apply_to(format!("Dependencies for {}:", pkg_name))
        );
        if diffdeps.is_empty() {
            println!("No runtimedependencies.");
        } else {
            print_vec_cols(diffdeps, Some(maxdeps as i32), 8);
        }

        println!(
            "{}",
            bold.apply_to(format!("Builddependencies for {}:", pkg_name))
        );
        if diffbdeps.is_empty() {
            println!("No builddependencies.");
        } else {
            print_vec_cols(diffbdeps, Some(maxbdeps as i32), 8);
        }

        println!(
            "{}",
            bold.apply_to(format!("Crossdependencies for {}:", pkg_name))
        );
        if diffcdeps.is_empty() {
            println!("No crossdependencies.");
        } else {
            print_vec_cols(diffcdeps, Some(maxcdeps as i32), 8);
        }
        Ok(())
    }

    //get extra sources
    pub fn get_extra_sources(&mut self) -> Result<(), std::io::Error> {
        debug!("Trying to list extra sources...");

        let bold = Style::new().bold();
        let italic = Style::new().italic();

        let resp = self.write_and_read("GET_MANAGED_EXTRA_SOURCES")?;
        let ess = serde_json::from_str::<Vec<ExtraSource>>(&resp)?;
        println!("{}", bold.apply_to("Managed Extra Sources:"));
        println!(
            "{}",
            italic.apply_to(format!("{:<40} {:<30} {}", "Id", "Filename", "Description"))
        );
        ess.iter().for_each(|es| {
            println!("{}", es);
        });
        Ok(())
    }

    // deletes a specified packagebuild
    pub fn delete_pkgbuild(&mut self, pkg_name: &str) -> Result<(), std::io::Error> {
        debug!("Trying to delete pkgbuild for {pkg_name}...");

        let resp = self.write_and_read(&format!("DELETE_PKGBUILD {}", pkg_name))?;

        match resp.as_str() {
            "CMD_OK" => Ok(()),
            "INV_CMD" => {
                error!("Command was invalid");
                self.exit_clean(-1)
            }
            "INV_PKG_NAME" => {
                error!("Package {pkg_name} does not exist");
                self.exit_clean(-1)
            }
            "REQUIRED_PKG" => {
                error!("Can't delte required package");
                self.exit_clean(-1)
            }
            other => {
                error!("Received unexpected response: {other}");
                self.exit_clean(-1)
            }
        }
    }
}
