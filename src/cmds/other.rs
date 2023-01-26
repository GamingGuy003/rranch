use std::{net::TcpStream, process::Command, time::Duration};

use log::{debug, error, info, trace, warn};

use crate::{
    cmds::{fetch::fetch_log_of, job::request_status},
    sockops::coms::write_and_read,
    structs::{job::Job, pkgbuild::PKGBuildJson},
    util::funcs::get_choice,
};

use super::{fetch::fetch_packagebuild_for, submit::submit_packagebuild};

pub fn latest_log(socket: &TcpStream) -> i32 {
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
    let last_id = match completed.last() {
        Some(last) => last.get_id(),
        None => {
            info!("No completed jobs.");
            return -1;
        }
    };

    fetch_log_of(socket, &last_id);
    0
}

pub fn watch_jobs(socket: &TcpStream, interval: &str) -> i32 {
    let n = interval.parse::<u64>().unwrap_or_else(|_| {
        warn!("Failed converting interval to u64; falling back to 5 secs");
        5
    });
    let mut i: u128 = 0;
    loop {
        i += 1;
        request_status(socket, true);
        info!("Update: {}", i);
        std::thread::sleep(Duration::from_secs(n));
    }
}

pub fn edit(socket: &TcpStream, pkg_name: &str, editor: &str) -> i32 {
    fetch_packagebuild_for(socket, pkg_name);
    let path = format!("{}/package.bpb", pkg_name);

    let child = Command::new(editor).arg(path.clone()).spawn();
    match child {
        Ok(mut child) => {
            let exit_status = match child.wait() {
                Ok(status) => status,
                Err(err) => {
                    error!("Failed to wait on child: {}", err);
                    return -1;
                }
            };
            if !exit_status.success() {
                error!("Editor closed with error");
                return -1;
            }
        }
        Err(_) => {
            error!("Editor {} not found", editor);
            return -1;
        }
    }
    if get_choice("Do you want to submit the changes", false) {
        submit_packagebuild(socket, path.as_str());
    } else {
        info!("Aborted submit due to user choice.");
    }
    if get_choice("Do you want to delete the local packagebuild", true) {
        match std::fs::remove_dir_all(pkg_name) {
            Ok(_) => {}
            Err(err) => {
                error!("Failed deleting locale packagebuild dir: {}", err);
                return -1;
            }
        }
    }
    0
}

pub fn create_template() -> i32 {
    PKGBuildJson::new_template().create_workdir()
}
