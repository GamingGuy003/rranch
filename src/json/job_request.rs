use serde_derive::Serialize;

#[derive(Serialize)]
pub struct JobRequest {
    pub jobid: String,
    pub offset: usize,
}

impl JobRequest {
    pub fn new(jobid: &str, offset: usize) -> Self {
        Self {
            jobid: jobid.to_owned(),
            offset,
        }
    }
}
