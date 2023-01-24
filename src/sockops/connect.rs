use std::{net::TcpStream, process::exit};

use log::{debug, error, info};

use crate::{sockops::coms::write_and_read, util::funcs::cleanup};

pub fn connect(
    host: &str,
    port: &str,
    name: &str,
    key: &str,
    r#type: &str,
) -> Result<TcpStream, i32> {
    info!(
        "Trying to set up master connection to {}:{}. Using client name {} with type {}...",
        host, port, name, r#type
    );
    //connect to master
    debug!("Connecting to master...");
    let socket = match TcpStream::connect(format!("{}:{}", host, port)) {
        Ok(socket) => {
            debug!("Successfully connected!");
            socket
        }
        Err(err) => {
            error!("Failed to establish connection: {}", err);
            return Err(-1);
        }
    };

    //authentication
    debug!("Trying to authenticate on master...");
    if key.is_empty() {
        error!("Invalid authkey");
        return Err(-1);
    }
    let resp = match write_and_read(&socket, format!("AUTH {}", key)) {
        Ok(msg) => msg,
        Err(err) => {
            error!("{}", err);
            return Err(-1);
        }
    };
    match resp.as_str() {
        "AUTH_OK" => debug!("Successfully authenticated!"),
        "UNTRUSTED_MODE" => info!("Running in untrusted mode!"),
        "INV_AUTH_KEY" => {
            error!("Failed to authenticate!");
            return Err(-1);
        }
        msg => {
            error!("Received unknown response from server: {}", msg);
            return Err(-1);
        }
    }

    //set machine type
    debug!("Trying to set machine type...");
    if r#type.is_empty() {
        error!("Invalid client type!");
        return Err(-1);
    }
    let resp = match write_and_read(&socket, format!("SET_MACHINE_TYPE {}", r#type)) {
        Ok(msg) => msg,
        Err(err) => {
            error!("{}", err);
            return Err(-1);
        }
    };

    match resp.as_str() {
        "CMD_OK" => debug!("Successfully set machien type!"),
        "AUTH_REQUIRED" => {
            error!("Not authenticated. Is authkey set?");
            return Err(-1);
        }
        "INV_MACHINE_TYPE" => {
            error!("Failed to set machine type!");
            return Err(-1);
        }
        msg => {
            error!("Received unknown response from server: {}", msg);
            return Err(-1);
        }
    }

    //set client name
    debug!("Trying to set machine type...");
    if name.is_empty() {
        error!("Invalid machine Type!");
        return Err(-1);
    }

    let resp = match write_and_read(&socket, format!("SET_MACHINE_NAME {}", name)) {
        Ok(msg) => msg,
        Err(err) => {
            error!("{}", err);
            return Err(-1);
        }
    };

    match resp.as_str() {
        "CMD_OK" => debug!("Successfully set machine name!"),
        msg => {
            error!("Received unknown response from server: {}", msg);
            cleanup(Some(socket), Some(-1));
            exit(-1)
        }
    }

    info!("Successfully set up connection!");
    Ok(socket)
}
