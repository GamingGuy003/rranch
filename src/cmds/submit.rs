use std::net::TcpStream;

use log::{debug, error, info};

use crate::{
    cmds::job::request_status, sockops::coms::write_and_read, structs::pkgbuild::PKGBuildJson,
    util::funcs::get_choice,
};

pub fn submit_solution(socket: &TcpStream, filename: &str, cb: bool) -> i32 {
    let cmd = if cb { "CB" } else { "RB" };

    let file = match std::fs::read_to_string(filename) {
        Ok(file) => {
            debug!("Successfully read sol file {}", filename);
            file
        }
        Err(err) => {
            error!("Error reading sol file: {}", err);
            return -1;
        }
    };

    let mut ret: Vec<Vec<String>> = Vec::new();
    file.lines().for_each(|line| {
        ret.push(
            line.split(';')
                .into_iter()
                .map(|value| value.to_owned())
                .collect(),
        )
    });
    let resp = match write_and_read(socket, format!("SUBMIT_SOLUTION_{} {:?}", cmd, ret)) {
        Ok(resp) => {
            debug!("Server responded with: {}", resp);
            resp
        }
        Err(err) => {
            error!("Error while communicating with server: {}", err);
            return -1;
        }
    };

    match resp.as_str() {
        s if s.starts_with("PKG_BUILD_MISSING") => {
            error!(
                "Missing packagebuild on server: {}",
                s.splitn(2, ' ').collect::<Vec<&str>>()[1]
            );
            -1
        }
        "BATCH_QUEUED" => {
            info!("Successfully queued solutionfile!");
            request_status(socket, false)
        }
        msg => {
            error!("Received unknown response from server: {}", msg);
            -1
        }
    }
}

pub fn submit_packagebuild(socket: &TcpStream, filename: &str) -> i32 {
    let pkgbuild = PKGBuildJson::from_pkgbuild(filename);

    let resp = match write_and_read(socket, "MANAGED_PKGBUILDS".to_owned()) {
        Ok(resp) => resp,
        Err(err) => {
            error!("Error communicating with server: {}", err);
            return -1;
        }
    };

    let pkgb: Vec<String> = serde_json::from_str(resp.as_str()).unwrap_or_default();
    if pkgb.contains(&pkgbuild.get_name())
        && !get_choice(
            "Packagebuild exists on remote. Do you want to overwrite it",
            false,
        )
    {
        error!("Aborted submit due to user choice");
        return -1;
    }

    let json = serde_json::to_string(&pkgbuild).unwrap_or_else(|_| "".to_owned());
    if json.is_empty() {
        error!("Failed to serialize struct! Check pkgbuild content...");
        return -1;
    }
    let resp = match write_and_read(socket, format!("SUBMIT_PACKAGE {}", json)) {
        Ok(resp) => {
            debug!("Successfully sent submit message");
            resp
        }
        Err(err) => {
            error!("Failed to send json to server: {}", err);
            return -1;
        }
    };
    match resp.as_str() {
        "INV_PKG_BUILD" => {
            error!("Package submission rejected by server. The package build you attempted to submit is invalid.");
            -1
        }
        "CMD_OK" => {
            info!("Package submission accepted by server.");
            0
        }
        msg => {
            error!("Received unknown message from server: {}", msg);
            -1
        }
    }
}
