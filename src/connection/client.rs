use std::{
    io::{self, Read, Write},
    net::TcpStream,
    process::exit,
};

use log::{debug, error, trace};

use crate::structs::pkgbuild::PKGBuildJson;

pub struct Client {
    socket: TcpStream,
    authkey: Option<String>,
    client_name: String,
    client_type: String,
}

impl Client {
    pub fn new(
        addr: &str,
        port: i32,
        authkey: Option<String>,
        client_name: String,
        client_type: String,
    ) -> Result<Self, io::Error> {
        let socket = match TcpStream::connect(format!("{addr}:{port}")) {
            Ok(socket) => socket,
            Err(err) => return Err(err),
        };

        Ok(Self {
            socket,
            authkey,
            client_name,
            client_type,
        })
    }

    pub fn close_connection(&self) -> Result<(), std::io::Error> {
        debug!("Trying to shut down socket...");
        self.socket.shutdown(std::net::Shutdown::Both)
    }

    pub fn exit_clean(&self, code: i32) -> Result<(), std::io::Error> {
        self.close_connection()?;
        exit(code)
    }

    pub fn auth(&mut self) -> Result<(), std::io::Error> {
        debug!("Trying to authenticate...");

        if self.authkey.is_none() {
            return Ok(());
        }

        let resp =
            self.write_and_read(format!("AUTH {}", self.authkey.clone().unwrap_or_default()))?;

        match resp.as_str() {
            "AUTH_OK" => Ok(()),
            "UNTRUSTED_MODE" => {
                debug!("Authenticated but running in untrusted mode");
                Ok(())
            }
            other => {
                error!("Received unexpected message: {other}");
                self.exit_clean(-1)
            }
        }
    }

    pub fn set_type(&mut self) -> Result<(), std::io::Error> {
        debug!("Trying to set machine type to {}...", self.client_type);

        let resp = self.write_and_read(format!("SET_MACHINE_TYPE {}", self.client_type))?;

        if resp == "CMD_OK" {
            return Ok(());
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Received: {resp}"),
            ));
        }
    }

    pub fn set_name(&mut self) -> Result<(), std::io::Error> {
        debug!("Trying to set machine name to {}...", self.client_name);

        let resp = self.write_and_read(format!("SET_MACHINE_NAME {}", self.client_name))?;

        if resp == "CMD_OK" {
            return Ok(());
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Received: {resp}"),
            ));
        }
    }

    fn read(&mut self) -> Result<String, std::io::Error> {
        let len = self.get_len()?;
        trace!("Trying to read {} bytes from socket...", len);
        let mut read = vec![0; len as usize];
        let ret = match self.socket.read_exact(&mut read) {
            Ok(_) => String::from_utf8(read.into_iter().collect()).unwrap_or_default(),
            Err(err) => return Err(err),
        };
        trace!("Received message was: {:?}", ret);
        Ok(ret)
    }

    fn write(&mut self, content: String) -> Result<(), std::io::Error> {
        trace!("Trying to write message to socket: {content}");

        match self
            .socket
            .write(format!("{} {}", content.as_bytes().len(), content).as_bytes())
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    fn get_len(&mut self) -> Result<i32, std::io::Error> {
        trace!("Trying to fetch message length...");
        let mut buffer = Vec::new();

        for byte in std::io::Read::by_ref(&mut self.socket).bytes() {
            let byte = byte?;
            if byte == b' ' {
                break;
            }
            buffer.push(byte);
        }

        match String::from_utf8(buffer).unwrap_or_default().parse::<i32>() {
            Ok(len) => Ok(len),
            Err(err) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("{}", err),
            )),
        }
    }

    pub fn write_and_read(&mut self, content: String) -> Result<String, std::io::Error> {
        self.write(content)?;
        self.read()
    }

    pub fn debug_shell(&mut self) -> Result<(), std::io::Error> {
        debug!("Starting debug shell...");
        println!("Type quit or enter to quit dbs");
        let mut send;
        loop {
            let mut user_input = String::new();
            print!("[rranch] ~> ");
            std::io::stdout().flush()?;
            std::io::stdin().read_line(&mut user_input)?;
            send = user_input.trim();

            //enter to quit dbs quit to quit program
            match send {
                "" | "quit" => return self.exit_clean(0),
                _ => {}
            }

            match self.write_and_read(send.to_owned()) {
                Ok(msg) => println!("{}", msg),
                Err(err) => {
                    error!("{}", err);
                    return self.exit_clean(-1);
                }
            };
        }
    }

    pub fn checkout(&mut self, pkgname: &str) -> Result<(), std::io::Error> {
        debug!("Trying to checkout {pkgname}...");

        let resp = self.write_and_read(format!("CHECKOUT_PACKAGE {}", pkgname))?;

        match resp.as_str() {
            "INV_PKG_NAME" => {
                error!("Invalid package name");
                self.exit_clean(-1)?;
            }
            "INV_PKG" => {
                error!("The packagebuild is invalid");
                self.exit_clean(-1)?;
            }
            _ => {}
        }

        match serde_json::from_str::<PKGBuildJson>(&resp) {
            Ok(json) => json.create_workdir(),
            Err(err) => {
                error!("Failed to deserialize json: {err}");
                self.exit_clean(-1)
            }
        }
    }

    pub fn submit(&mut self) {}
    pub fn release(&mut self) {}
    pub fn cross(&mut self) {}
    pub fn build_status(&mut self) {}
    pub fn client_status(&mut self) {}
    pub fn cancel_job(&mut self) {}
    pub fn cancel_jobs(&mut self) {}
    pub fn sys_log(&mut self) {}
    pub fn build_log(&mut self) {}
    pub fn clear_completed(&mut self) {}
    pub fn get_packages(&mut self) {}
    pub fn get_packagebuilds(&mut self) {}
    pub fn get_dependers(&mut self) {}
    pub fn get_dependencies(&mut self) {}
    pub fn rebuild_dependers(&mut self) {}
    pub fn get_diff(&mut self) {}
    pub fn solution_release_build(&mut self) {}
    pub fn solution_cross_build(&mut self) {}
    pub fn solution(&mut self) {}
    pub fn edit(&mut self) {}
    pub fn export(&mut self) {}
    pub fn import(&mut self) {}
    pub fn client_info(&mut self) {}
    pub fn submit_extra_source(&mut self) {}
    pub fn get_extra_sources(&mut self) {}
    pub fn remove_extra_source(&mut self) {}
}
