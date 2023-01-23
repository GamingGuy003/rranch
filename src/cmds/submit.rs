use std::{
    net::{Shutdown, TcpStream},
    process::exit,
};

use log::{debug, error, info, trace};

use crate::{coms::coms::write_and_read, structs::pkgbuild::PKGBuildJson, util::util::get_choice};

pub fn submit_solution(socket: &TcpStream, filename: &str, cb: bool) {
    let cmd;
    if cb {
        cmd = "SUBMIT_SOLUTION_CB";
    } else {
        cmd = "SUBMIT_SOLUTION_RB";
    }

    let file = match std::fs::read_to_string(filename) {
        Ok(file) => {
            debug!("Successfully read sol file {}", filename);
            file
        }
        Err(err) => {
            error!("Error reading sol file: {}", err);
            match socket.shutdown(std::net::Shutdown::Both) {
                Ok(_) => {}
                Err(err) => trace!("Failed to close socket: {}", err),
            }
            exit(-1)
        }
    };

    let mut ret: Vec<Vec<String>> = Vec::new();
    file.lines().for_each(|line| {
        ret.push(
            line.split(";")
                .into_iter()
                .map(|value| value.to_owned())
                .collect(),
        )
    });
    let resp = match write_and_read(socket, format!("{} {:?}", cmd, ret)) {
        Ok(resp) => {
            debug!("Server responded with: {}", resp);
            resp
        }
        Err(err) => {
            error!("Error while communicating with server: {}", err);
            match socket.shutdown(std::net::Shutdown::Both) {
                Ok(_) => {}
                Err(err) => trace!("Failed to close socket: {}", err),
            }
            exit(-1)
        }
    };

    match resp.as_str() {
        s if s.starts_with("PKG_BUILD_MISSING") => {
            error!(
                "Missing packagebuild on server: {}",
                s.splitn(2, " ").collect::<Vec<&str>>()[1]
            );
            match socket.shutdown(std::net::Shutdown::Both) {
                Ok(_) => {}
                Err(err) => trace!("Failed to close socket: {}", err),
            }
            exit(-1)
        }
        "BATCH_QUEUED" => info!("Successfully queued solutionfile!"),
        msg => {
            error!("Received unknown response from server: {}", msg);
            match socket.shutdown(std::net::Shutdown::Both) {
                Ok(_) => {}
                Err(err) => trace!("Failed to close socket: {}", err),
            }
            exit(-1)
        }
    }
}

pub fn submit_packagebuild(socket: &TcpStream, filename: &str) {
    let pkgbuild = PKGBuildJson::from_pkgbuild(filename);

    let resp = match write_and_read(socket, "MANAGED_PKGBUILDS".to_owned()) {
        Ok(resp) => resp,
        Err(err) => {
            error!("Error communicating with server: {}", err);
            match socket.shutdown(std::net::Shutdown::Both) {
                Ok(_) => {}
                Err(err) => trace!("Failed to close socket: {}", err),
            }
            exit(-1)
        }
    };

    let pkgb: Vec<String> = serde_json::from_str(resp.as_str()).unwrap_or(Vec::new());
    if pkgb.contains(&pkgbuild.get_name()) {
        if !get_choice("Packagebuild exists on remote. Do you want to overwrite it?") {
            error!("Aborted submit due to user choice");
            match socket.shutdown(std::net::Shutdown::Both) {
                Ok(_) => {}
                Err(err) => trace!("Failed to close socket: {}", err),
            }
            exit(-1)
        }
    }

    let json = serde_json::to_string(&pkgbuild).unwrap_or("".to_owned());
    if json.len() == 0 {
        error!("Failed to serialize struct! Check pkgbuild content...");
        socket
            .shutdown(Shutdown::Both)
            .unwrap_or(trace!("Failed to close socket"));
        exit(-1)
    }
    let resp = match write_and_read(socket, format!("SUBMIT_PACKAGE {}", json)) {
        Ok(resp) => {
            debug!("Successfully sent submit message");
            resp
        }
        Err(err) => {
            error!("Failed to send json to server: {}", err);
            match socket.shutdown(std::net::Shutdown::Both) {
                Ok(_) => {}
                Err(err) => trace!("Failed to close socket: {}", err),
            }
            exit(-1)
        }
    };
    match resp.as_str() {
        "INV_PKG_BUILD" => {
            error!("Package submission rejected by server. The package build you attempted to submit is invalid.");
            match socket.shutdown(std::net::Shutdown::Both) {
                Ok(_) => {}
                Err(err) => trace!("Failed to close socket: {}", err),
            }
            exit(-1)
        }
        "CMD_OK" => {
            info!("Package submission accepted by server.");
        }
        msg => {
            error!("Received unknown message from server: {}", msg);
            match socket.shutdown(std::net::Shutdown::Both) {
                Ok(_) => {}
                Err(err) => trace!("Failed to close socket: {}", err),
            }
            exit(-1)
        }
    }
}
