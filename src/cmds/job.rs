use std::net::TcpStream;

use console::{Style, Term};
use log::{debug, error, info, trace};

use crate::{sockops::coms::write_and_read, structs::job::Job};

pub fn request_status(socket: &TcpStream, clear: bool) -> i32 {
    let bold = Style::new().bold();
    let ital = Style::new().italic();

    //running jobs
    let resp = match write_and_read(socket, "RUNNING_JOBS_STATUS".to_owned()) {
        Ok(resp) => {
            debug!("Successfully fetched running jobs from server");
            resp
        }
        Err(err) => {
            error!("Encountered error while communicating with server: {}", err);
            return -1;
        }
    };

    let running = serde_json::from_str::<Vec<Job>>(&resp).unwrap_or_default();
    trace!("Successfully received and parsed running jobs");

    //completed jobs
    let resp = match write_and_read(socket, "COMPLETED_JOBS_STATUS".to_owned()) {
        Ok(resp) => {
            debug!("Successfully fetched completed jobs from server");
            resp
        }
        Err(err) => {
            error!("Encountered error while communicating with server: {}", err);
            return -1;
        }
    };

    let completed = serde_json::from_str::<Vec<Job>>(&resp).unwrap_or_default();
    trace!("Successfully received and parsed completed jobs");

    //queued jobs
    let resp = match write_and_read(socket, "QUEUED_JOBS_STATUS".to_owned()) {
        Ok(resp) => {
            debug!("Successfully fetched queued jobs from server");
            resp
        }
        Err(err) => {
            error!("Encountered error while communicating with server: {}", err);
            return -1;
        }
    };

    let queued = serde_json::from_str::<Vec<Job>>(&resp).unwrap_or_default();
    trace!("Successfully received and parsed queued jobs");

    if clear {
        Term::stdout().clear_screen().unwrap_or(());
        Term::stdout().move_cursor_to(0, 0).unwrap_or(());
    }

    println!("{}", bold.apply_to("RUNNING JOBS"));
    if running.is_empty() {
        println!("No jobs.");
    } else {
        println!(
            "{}",
            ital.apply_to(format!(
                "{:<20} {:<15} {:<40} {:10}",
                "NAME", "STATUS", "ID", "REQUESTED BY"
            ))
        );
        for job in running {
            println!("{}", job);
        }
    }

    println!("{}", bold.apply_to("COMPLETED JOBS"));
    if completed.is_empty() {
        println!("No jobs.");
    } else {
        println!(
            "{}",
            ital.apply_to(format!(
                "{:<20} {:<15} {:<40} {:10}",
                "NAME", "STATUS", "ID", "REQUESTED BY"
            ))
        );
        for job in completed {
            println!("{}", job);
        }
    }

    println!("{}", bold.apply_to("QUEUED JOBS"));
    if queued.is_empty() {
        println!("No jobs.");
    } else {
        println!(
            "{}",
            ital.apply_to(format!(
                "{:<20} {:<15} {:<40} {:10}",
                "NAME", "STATUS", "ID", "REQUESTED BY"
            ))
        );
        for job in queued {
            println!("{}", job);
        }
    }
    0
}

pub fn request_cancel_queued_job(socket: &TcpStream, job_id: &str) -> i32 {
    let resp = match write_and_read(socket, format!("CANCEL_QUEUED_JOB {}", job_id)) {
        Ok(resp) => {
            debug!(
                "Successfully fetched response for CANCEL_QUEUD_JOB: {}",
                resp
            );
            resp
        }
        Err(err) => {
            error!("Error communicating with server: {}", err);
            return -1;
        }
    };

    match resp.as_str() {
        "INV_JOB_ID" => {
            error!("No such job queued");
            return -1;
        }
        "JOB_CANCELED" => info!("Successfully canceled job {}", job_id),
        msg => {
            error!("Received unknown response: {}", msg);
            return -1;
        }
    }
    request_status(socket, false);
    0
}

pub fn request_clear_completed_jobs(socket: &TcpStream) -> i32 {
    let resp = match write_and_read(socket, "CLEAR_COMPLETED_JOBS".to_owned()) {
        Ok(resp) => {
            debug!(
                "Successfully fetched response for CLEAR_COMPLETED_JOBS: {}",
                resp
            );
            resp
        }
        Err(err) => {
            error!("Error communicating with server: {}", err);
            return -1;
        }
    };

    if resp.as_str() != "JOBS_CLEARED" {
        error!("Failed to clear completed jobs: {}", resp);
        return -1;
    } else {
        info!("Successfully cleared jobs");
    }
    request_status(socket, false);
    0
}

pub fn request_cancel_all_jobs(socket: &TcpStream) -> i32 {
    let resp = match write_and_read(socket, "CANCEL_ALL_QUEUED_JOBS".to_owned()) {
        Ok(resp) => {
            debug!(
                "Successfully fetched response for CANCEL_ALL_QUEUD_JOBS: {}",
                resp
            );
            resp
        }
        Err(err) => {
            error!("Error communicating with server: {}", err);
            return -1;
        }
    };

    if resp.as_str() != "JOBS_CANCELED" {
        error!("Failed to cancel all queued jobs: {}", resp);
        return -1;
    } else {
        info!("Successfully cancelled all queued jobs");
    }
    request_status(socket, false);
    0
}

pub fn request_rebuild_dependers(socket: &TcpStream, pkg_name: &str) -> i32 {
    let resp = match write_and_read(socket, format!("REBUILD_DEPENDERS {}", pkg_name)) {
        Ok(resp) => {
            debug!("Successfully requested dependers rebuild for {}", pkg_name);
            resp
        }
        Err(err) => {
            error!("Error while communicating with server: {}", err);
            return -1;
        }
    };

    match resp.as_str() {
        "INV_PKG_NAME" => {
            error!("No such package available");
            -1
        }
        "CIRCULAR_DEPENDENCY" => {
            error!("Circular dependency detected. The requested batch build contains a circular dependency and could not be submitted.");
            -1
        }
        "BATCH_QUEUED" => {
            info!("Successfully queued batch");
            request_status(socket, false)
        }
        msg => {
            error!("Received unknown response from server: {}", msg);
            -1
        }
    }
}

pub fn request_build(socket: &TcpStream, pkg_name: &str, cb: bool) -> i32 {
    let cmd = if cb { "CROSS_BUILD" } else { "RELEASE_BUILD" };

    let binding = match write_and_read(socket, format!("{} {}", cmd, pkg_name)) {
        Ok(resp) => resp,
        Err(err) => {
            error!("Encountered error while communicating with server: {}", err);
            return -1;
        }
    };
    let resp = binding.as_str();

    match resp {
        "BUILD_REQ_SUBMIT_IMMEDIATELY" => {
            info!("The package build was immediately handled by a ready build bot.");
            0
        }
        "BUILD_REQ_QUEUED" => {
            info!("No buildbot is currently available to handle the build request. Build request added to queue.");
            request_status(socket, false)
        }
        "INV_PKG_NAME" => {
            error!("Invalid package name!");
            -1
        }
        "PKG_BUILD_DAMAGED" => {
            error!("The packagebuild you attempted to queue is damaged.");
            -1
        }
        msg => {
            error!("Received invalid response from server: {}", msg);
            -1
        }
    }
}
