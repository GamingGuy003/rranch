use super::client::Client;

impl Client {
    // gets job status
    pub fn build_status(&mut self) {}
    // gets client status
    pub fn client_status(&mut self) {}
    // gets info of a client
    pub fn client_info(&mut self, client_name: &str) {}

    // gets sys log
    pub fn sys_log(&mut self) {}
    // gets build log
    pub fn build_log(&mut self, job_id: &str) {}
}
