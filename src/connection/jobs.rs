use std::{path::Path, process::Command};

use console::Term;
use indicatif::ProgressBar;
use log::{debug, error, info};

use crate::{
    json::extra_sources::{ExtraSource, ExtraSourceSubmit},
    util::funcs::{get_choice, get_input, get_pkgbs},
};

use super::client::Client;

impl Client {
    // requests release / crossbuild for pkg
    pub fn build(&mut self, rb: bool, pkg_name: &str) -> Result<(), std::io::Error> {
        debug!("Trying to request build: {pkg_name}; rb: {rb}...");

        let cmd = if rb { "RELEASE_BUILD" } else { "CROSS_BUILD" };
        let resp = self.write_and_read(&format!("{cmd} {pkg_name}"))?;

        match resp.as_str() {
            "BUILD_REQ_SUBMIT_IMMEDIATELY" => {
                info!("The package build was immediately handled by a ready build bot.");
                Ok(())
            }
            "BUILD_REQ_QUEUED" => {
                info!("No buildbot is currently available to handle the build request. Build request added to queue.");
                Ok(())
            }
            "INV_PKG_NAME" => {
                error!("Invalid package name!");
                self.exit_clean(-1)
            }
            "PKG_BUILD_DAMAGED" => {
                error!("The packagebuild you attempted to queue is damaged.");
                self.exit_clean(-1)
            }
            other => {
                error!("Received invalid response from server: {other}");
                self.exit_clean(-1)
            }
        }
    }

    // cancels specific job
    pub fn cancel_job(&mut self, job_id: &str) -> Result<(), std::io::Error> {
        debug!("Trying to cancel job {job_id}...");

        let resp = self.write_and_read(&format!("CANCEL_QUEUED_JOB {job_id}"))?;

        match resp.as_str() {
            "JOB_CANCELED" => Ok(()),
            "INV_JOB_ID" => {
                error!("Job does not exist");
                self.exit_clean(-1)
            }
            other => {
                error!("Received unexpected response: {other}");
                self.exit_clean(-1)
            }
        }
    }

    // cancels all queued jobs
    pub fn cancel_all_jobs(&mut self) -> Result<(), std::io::Error> {
        debug!("Trying to cancel all jobs...");

        let resp = self.write_and_read("CANCEL_ALL_QUEUED_JOBS")?;

        match resp.as_str() {
            "JOBS_CANCELED" => Ok(()),
            other => {
                error!("Received unexpected response: {other}");
                self.exit_clean(-1)
            }
        }
    }

    // clears completed jobs
    pub fn clear_completed(&mut self) -> Result<(), std::io::Error> {
        debug!("Trying to clear completed jobs...");

        let resp = self.write_and_read("CLEAR_COMPLETED_JOBS")?;

        match resp.as_str() {
            "JOBS_CLEARED" => Ok(()),
            other => {
                error!("Received unexpected response: {other}");
                self.exit_clean(-1)
            }
        }
    }

    // requests dependers rebuild for pkg
    pub fn rebuild_dependers(&mut self, pkg_name: &str) -> Result<(), std::io::Error> {
        debug!("Trying to rebuild dependers for {pkg_name}...");

        let resp = self.write_and_read(&format!("REBUILD_DEPENDERS {pkg_name}"))?;

        match resp.as_str() {
            "CMD_OK" => Ok(()),
            "INV_PKG_NAME" => {
                error!("Invalid Package name");
                self.exit_clean(-1)
            }
            "RELEASE_ENV_UNAVAILABLE" => {
                error!("Release enviroment is unavailable");
                self.exit_clean(-1)
            }
            "CROSS_ENV_UNAVAILABLE" => {
                error!("Cross enviroment is unavailable");
                self.exit_clean(-1)
            }
            other => {
                error!("Received unexpected response: {other}");
                self.exit_clean(-1)
            }
        }
    }

    // opens editor for pkg
    pub fn edit(&mut self, pkg_name: &str, editor: &str) -> Result<(), std::io::Error> {
        debug!("Trying to edit {pkg_name}");

        self.checkout(pkg_name)?;
        let path = format!("{}/package.bpb", pkg_name);
        let child = Command::new(editor).arg(path.clone()).spawn();

        match child {
            Ok(mut child) => {
                if !child.wait()?.success() {
                    error!("Editor closed with error");
                    return self.exit_clean(-1);
                }
            }
            Err(_) => {
                error!("Editor {} not found", editor);
                return self.exit_clean(-1);
            }
        }

        if get_choice("Do you want to submit the changes", false)? {
            self.submit(path.as_str())?;
        } else {
            info!("Aborted submit due to user choice.");
        }

        if get_choice("Do you want to delete the local packagebuild", true)? {
            std::fs::remove_dir_all(pkg_name)?;
        }
        Ok(())
    }
    // exports all pkgbs from server
    pub fn export_all(&mut self) -> Result<(), std::io::Error> {
        debug!("Trying to export all pkgbs...");

        let mut pkgbs =
            serde_json::from_str::<Vec<String>>(&self.write_and_read("MANAGED_PKGBUILDS")?)?;
        pkgbs.sort();
        if !get_choice(
            &format!("Do you want to fetch {} pkgbuilds", pkgbs.len()),
            false,
        )? {
            debug!("Aborted due to user choice");
            return Ok(());
        }
        let progress = ProgressBar::new(pkgbs.len() as u64);
        let term = Term::stdout();
        for pkgb in pkgbs {
            term.clear_screen()?;
            progress.inc(1);
            self.checkout(&pkgb)?;
        }
        progress.finish();
        term.clear_screen()?;
        Ok(())
    }

    // imports and submits all pkgs from folder to server
    pub fn import_folder(&mut self, path: &str) -> Result<(), std::io::Error> {
        debug!("Trying to import all pkgbs from {path}...");

        let pkgbs = get_pkgbs(path)?;
        if !get_choice(
            &format!("Do you want to submit {} pkgbuilds", pkgbs.len()),
            false,
        )? {
            debug!("Aborted due to user choice");
            return Ok(());
        }
        let progress = ProgressBar::new(pkgbs.len() as u64);
        let term = Term::stdout();
        for pkgb in pkgbs {
            term.clear_screen()?;
            progress.inc(1);
            self.submit(&pkgb)?;
        }
        term.clear_screen()?;
        progress.finish();
        Ok(())
    }

    // submits extra source
    pub fn submit_extra_source(&mut self, path: &str) -> Result<(), std::io::Error> {
        debug!("Trying to submit extrasource...");

        let path = Path::new(path);

        if !path.is_file() {
            error!("Path does not lead to a file");
            return self.exit_clean(-1);
        }
        print!(
            "Description for {}: ",
            path.as_os_str().to_str().unwrap_or_default()
        );
        let description = get_input()?;
        let file = std::fs::read(path)?;
        let json = serde_json::to_string(&ExtraSourceSubmit::new(
            &description,
            path.file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default(),
            &file.len().to_string(),
        ))?;

        let resp = self.write_and_read(&format!("TRANSFER_EXTRA_SOURCE {}", json))?;

        match resp.as_str() {
            "CMD_OK" => {}
            "BYTE_COUNT_ERR" => {
                error!("Byte count invalid");
                return self.exit_clean(-1);
            }
            other => {
                error!("Received unexpected response: {other}");
                return self.exit_clean(-1);
            }
        }

        self.write_raw(file)?;

        let resp = self.write_and_read("COMPLETE_TRANSFER")?;

        match resp.as_str() {
            "UPLOAD_ACK" => Ok(()),
            "ERR_COULD_NOT_INSERT" => {
                error!("Failed to insert on remote");
                self.exit_clean(-1)
            }
            other => {
                error!("Receivede unexpected response: {other}");
                self.exit_clean(-1)
            }
        }
    }
    // removes extra source
    pub fn remove_extra_source(&mut self, es_id: &str) -> Result<(), std::io::Error> {
        debug!("Trying to remove {es_id}...");

        let resp = self.write_and_read(&format!("REMOVE_EXTRA_SOURCE {}", es_id))?;

        match resp.as_str() {
            "CMD_OK" => Ok(()),
            "INV_ES_ID" => {
                error!("Extra source id was invalid");
                self.exit_clean(-1)
            }
            other => {
                error!("Receivede unexpected response: {other}");
                self.exit_clean(-1)
            }
        }
    }

    // removes all extra sources
    pub fn remove_all_extra_sources(&mut self) -> Result<(), std::io::Error> {
        debug!("Trying to delete all extra sources");

        let ess = serde_json::from_str::<Vec<ExtraSource>>(
            &self.write_and_read("GET_MANAGED_EXTRA_SOURCES")?,
        )?;

        for es in ess {
            self.remove_extra_source(&es.id)?;
        }
        Ok(())
    }
}
