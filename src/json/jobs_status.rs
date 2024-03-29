use std::fmt::Display;

use console::Style;
use serde_derive::Deserialize;

use crate::util::funcs::truncate_to;

#[derive(Deserialize, Default)]
pub struct JobsStatus {
    pub queuedjobs: Vec<Job>,
    pub runningjobs: Vec<Job>,
    pub completedjobs: Vec<Job>,
}

#[derive(Deserialize, Default)]
pub struct Job {
    pub job_id: String,
    pub job_status: String,
    pub job_name: String,
    pub requesting_client: String,
}

impl Display for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let style = match self.job_status.as_str() {
            "COMPLETED" => Style::new().green(),
            "FAILED" | "BUILD_FAILED" => Style::new().red(),
            _ => Style::new().yellow(),
        };

        write!(
            f,
            "{:<20} {:<40} {:<20} {}",
            truncate_to(self.job_name.clone(), 18),
            self.job_id,
            truncate_to(self.requesting_client.clone(), 18),
            style.apply_to(self.job_status.clone())
        )
    }
}

impl JobsStatus {
    pub fn header(&self) -> String {
        let italic = Style::new().italic();
        format!(
            "{:<20} {:<40} {:<20} {:<20}",
            italic.apply_to("NAME"),
            italic.apply_to("ID"),
            italic.apply_to("REQUESTER"),
            italic.apply_to("STATUS"),
        )
    }
}

impl Display for JobsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bold = Style::new().bold();
        let queued = self.queuedjobs.iter().map(|job| job.to_string()).collect::<Vec<String>>().join("\n");
        let running = self.runningjobs.iter().map(|job| job.to_string()).collect::<Vec<String>>().join("\n");
        let completed = self.completedjobs.iter().map(|job| job.to_string()).collect::<Vec<String>>().join("\n");
        write!(
            f,
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            bold.apply_to("Queud Jobs"),
            self.header(),
            queued,
            bold.apply_to("Running Jobs"),
            self.header(),
            running,
            bold.apply_to("Completed Jobs"),
            self.header(),
            completed
        )
    }
}
