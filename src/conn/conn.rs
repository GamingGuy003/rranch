use std::{net::TcpStream, process::exit};

use log::{debug, error, info};

use crate::coms::coms::write_and_read;

pub fn connect(
    host: &str,
    port: &str,
    name: &str,
    key: &str,
    ctype: &str,
) -> Result<TcpStream, i32> {
    info!(
        "Trying to set up master connection to {}:{}. Using client name {} with type {}...",
        host, port, name, ctype
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
    if key.len() != 0 {
        let auth = match write_and_read(&socket, format!("AUTH {}", key)) {
            Ok(msg) => msg,
            Err(err) => {
                error!("{}", err);
                return Err(-1);
            }
        };
        match auth.as_str() {
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
    }

    //set machine type
    debug!("Trying to set machine type...");
    if ctype.len() == 0 {
        error!("Invalid client type!");
        return Err(-1);
    }

    let client = match write_and_read(&socket, format!("SET_MACHINE_TYPE {}", ctype)) {
        Ok(msg) => msg,
        Err(err) => {
            error!("{}", err);
            return Err(-1);
        }
    };

    match client.as_str() {
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
    debug!("Trying to set machine name...");
    if ctype.len() == 0 {
        error!("Invalid machine name!");
        return Err(-1);
    }

    let client = match write_and_read(&socket, format!("SET_MACHINE_NAME {}", name)) {
        Ok(msg) => msg,
        Err(err) => {
            error!("{}", err);
            return Err(-1);
        }
    };

    if client == "CMD_OK" {
        debug!("Successfully set machine name!");
    } else {
        error!("Failed to set machine name!");
        exit(-1)
    }

    info!("Successfully set up connection!");
    Ok(socket)
}
