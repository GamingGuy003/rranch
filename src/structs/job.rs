use console::Style;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Job {
    build_pkg_name: String,
    job_status: String,
    job_id: String,
    requesting_client: String,
}

impl Job {
    pub fn to_string(&self) -> String {
        let color = match self.job_status.as_str() {
            "FAILED" => Style::new().red(),
            "COMPLETED" => Style::new().green(),
            "WAITING" => Style::new().yellow(),
            _ => Style::new().yellow(),
        };
        format!(
            "{:<20} {:<15} {:<40} {:10}",
            self.build_pkg_name,
            color.apply_to(self.job_status.clone()),
            self.job_id,
            self.requesting_client
        )
    }

    pub fn get_id(&self) -> String {
        self.job_id.clone()
    }
}
