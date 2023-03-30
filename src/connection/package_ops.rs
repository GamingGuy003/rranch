use log::{debug, error};

use crate::structs::pkgbuild::PKGBuildJson;

use super::client::Client;

impl Client {
    // downloads pkgb and creates workdir
    pub fn checkout(&mut self, pkgname: &str) -> Result<(), std::io::Error> {
        debug!("Trying to checkout {pkgname}...");

        let resp = self.write_and_read(format!("CHECKOUT_PACKAGE {}", pkgname))?;

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

        match serde_json::from_str::<PKGBuildJson>(&resp) {
            Ok(json) => json.create_workdir(),
            Err(err) => {
                error!("Failed to deserialize json: {err}");
                self.exit_clean(-1)
            }
        }
    }

    // submits package
    pub fn submit(&mut self, path: &str) {}
    // submits solution and builds it (release / cross)
    pub fn submit_sol(&mut self, rb: bool, path: &str) {}
    // get list of pkgs
    pub fn get_packages(&mut self) {}
    // get list of pkgbs
    pub fn get_packagebuilds(&mut self) {}
    // get diff pkgbs / pkgs
    pub fn get_diff(&mut self) {}
    // get dependers of pkg
    pub fn get_dependers(&mut self) {}
    // get dependencies of pkg
    pub fn get_dependencies(&mut self) {}
    //get extra sources
    pub fn get_extra_sources(&mut self) {}
}
