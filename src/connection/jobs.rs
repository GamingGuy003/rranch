use log::{debug, error, info};

use super::client::Client;

impl Client {
    // requests release / crossbuild for pkg
    pub fn build(&mut self, rb: bool, pkg: &str) -> Result<(), std::io::Error> {
        debug!("Trying to request build; Release: {rb}");

        let cmd = if rb { "RELEASE_BUILD" } else { "CROSS_BUILD" };

        let resp = self.write_and_read(format!("{cmd} {pkg}"))?;

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
            msg => {
                error!("Received invalid response from server: {}", msg);
                self.exit_clean(-1)
            }
        }
    }

    // cancels specific job
    pub fn cancel_job(&mut self, job_id: &str) -> Result<(), std::io::Error> {
        debug!("Trying to cancel job {job_id}...");

        let resp = self.write_and_read(format!("CANCEL_QUEUED_JOB {job_id}"))?;

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

        self.write_and_read("CANCEL_ALL_QUEUED_JOBS".to_owned())?;

        Ok(())
    }

    // clears completed jobs
    pub fn clear_completed(&mut self) -> Result<(), std::io::Error> {
        debug!("Trying to clear completed jobs...");

        let resp = self.write_and_read("CLEAR_COMPLETED_JOBS".to_owned())?;
        Ok(())
    }

    // requests dependers rebuild for pkg
    pub fn rebuild_dependers(&mut self, pkg: &str) {}
    // opens editor for pkg
    pub fn edit(&mut self, pkg: &str) {}
    // exports all pkgbs from server
    pub fn export_all(&mut self) {}
    // imports and submtis all pkgs from folder to server
    pub fn import_folder(&mut self, path: &str) {}
    // submtis extra source
    pub fn submit_extra_source(&mut self, path: &str) {}
    // removes extra source
    pub fn remove_extra_source(&mut self, es_name: &str) {}
}
