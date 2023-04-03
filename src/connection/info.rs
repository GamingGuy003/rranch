use console::Style;
use log::{debug, error};

use crate::{json::job::Job, util::funcs::print_job_table};

use super::client::Client;

impl Client {
    // gets job status
    pub fn build_status(&mut self) -> Result<(), std::io::Error> {
        debug!("Trying to fetch job status");

        let bold = Style::new().bold();

        let completed =
            serde_json::from_str::<Vec<Job>>(&self.write_and_read("COMPLETED_JOBS_STATUS")?)?;
        let queued = serde_json::from_str::<Vec<Job>>(&self.write_and_read("QUEUED_JOBS_STATUS")?)?;
        let running =
            serde_json::from_str::<Vec<Job>>(&self.write_and_read("RUNNING_JOBS_STATUS")?)?;

        println!("{}", bold.apply_to("Queued Jobs"));
        if queued.is_empty() {
            println!("No queued jobs");
        } else {
            print_job_table(queued);
        }

        println!("{}", bold.apply_to("Running Jobs"));
        if running.is_empty() {
            println!("No running jobs");
        } else {
            print_job_table(running);
        }
        println!("{}", bold.apply_to("Completed Jobs"));
        if completed.is_empty() {
            println!("No completed jobs");
        } else {
            print_job_table(completed);
        }
        Ok(())
    }

    // gets client status
    pub fn client_status(&mut self) -> Result<(), std::io::Error> {
        debug!("Trying to fetch client status");

        let bold = Style::new().bold();

        let controllers =
            serde_json::from_str::<Vec<String>>(&self.write_and_read("CONNECTED_CONTROLLERS")?)?;
        let buildbots =
            serde_json::from_str::<Vec<String>>(&self.write_and_read("CONNECTED_BUILDBOTS")?)?;

        println!("{}", bold.apply_to("Connected controllers"));
        controllers
            .iter()
            .for_each(|controller| println!("{controller}"));
        println!("{}", bold.apply_to("Connected buildbots"));
        buildbots.iter().for_each(|buildbot| println!("{buildbot}"));
        Ok(())
    }
    // gets info of a client
    pub fn client_info(&mut self, client_name: &str) -> Result<(), std::io::Error> {
        debug!("Trying to fetch client info for {client_name}...");

        let json = serde_json::from_str::<serde_json::Value>(
            &self.write_and_read(&format!("GET_CLIENT_INFO {}", client_name))?,
        )?;

        println!("{:#}", json);

        Ok(())
    }

    // gets sys log
    pub fn sys_log(&mut self) -> Result<(), std::io::Error> {
        debug!("Trying to show syslog...");

        let events = serde_json::from_str::<Vec<String>>(&self.write_and_read("VIEW_SYS_EVENTS")?)?;

        events.iter().for_each(|event| println!("{event}"));
        Ok(())
    }
    // gets build log
    pub fn build_log(&mut self, job_id: &str) -> Result<(), std::io::Error> {
        debug!("Trying to show build log for {job_id}...");

        let resp = self.write_and_read(&format!("VIEW_LOG {}", job_id))?;

        match resp.as_str() {
            "INV_JOB_ID" => {
                error!("Job does not exist");
                self.exit_clean(-1)
            }
            "NO_LOG" => {
                println!("No log available");
                Ok(())
            }
            other => {
                match serde_json::from_str::<Vec<String>>(other) {
                    Ok(log) => log.iter().for_each(|line| println!("{line}")),
                    Err(_) => {
                        error!("Could not deserialize log; Received: {other}");
                        return self.exit_clean(-1);
                    }
                };
                Ok(())
            }
        }
    }
}
