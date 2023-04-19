use std::time::Duration;

use crate::{
    json::{
        jobs_status::{Job, JobsStatus},
        request::Request,
        response::{Response, StatusCode},
    },
    structs::client::Client,
};

impl Client {
    pub fn get_latest_log(&mut self) -> Result<(), std::io::Error> {
        let req = Request::new("GETJOBSTATUS", None);

        let resp =
            serde_json::from_str::<Response>(&self.write_read(&serde_json::to_string(&req)?)?)?;

        match resp.statuscode {
            StatusCode::Ok => self.get_job_log(
                serde_json::from_value::<JobsStatus>(resp.payload)?
                    .completedjobs
                    .last()
                    .unwrap_or(&Job::default())
                    .job_id
                    .as_str(),
            ),
            StatusCode::InternalServerError | StatusCode::RequestFailure => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    serde_json::to_string(&resp.payload)?,
                ))
            }
        }
    }

    pub fn watch_jobs(&mut self, interval: &str) -> Result<(), std::io::Error> {
        let n = interval.parse::<u64>().unwrap_or(1);
        loop {
            self.get_jobs_status(true)?;
            std::thread::sleep(Duration::from_secs(n));
        }
    }
}
