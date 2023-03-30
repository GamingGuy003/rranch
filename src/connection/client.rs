use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

use log::trace;

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
        self.socket.shutdown(std::net::Shutdown::Both)
    }

    pub fn auth(&mut self) -> Result<(), std::io::Error> {
        if self.authkey.is_none() {
            return Ok(());
        }

        let resp =
            self.write_and_read(format!("AUTH {}", self.authkey.clone().unwrap_or_default()))?;

        if resp == "AUTH_OK" {
            return Ok(());
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                format!("Received: {resp}"),
            ));
        }
    }

    pub fn set_type(&mut self) -> Result<(), std::io::Error> {
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

    pub fn debug_shell(&self) {}
    pub fn checkout(&self) {}
    pub fn submit(&self) {}
    pub fn release(&self) {}
    pub fn cross(&self) {}
    pub fn build_status(&self) {}
    pub fn client_status(&self) {}
    pub fn cancel_job(&self) {}
    pub fn cancel_jobs(&self) {}
    pub fn sys_log(&self) {}
    pub fn build_log(&self) {}
    pub fn clear_completed(&self) {}
    pub fn get_packages(&self) {}
    pub fn get_packagebuilds(&self) {}
    pub fn get_dependers(&self) {}
    pub fn get_dependencies(&self) {}
    pub fn rebuild_dependers(&self) {}
    pub fn get_diff(&self) {}
    pub fn solution_release_build(&self) {}
    pub fn solution_cross_build(&self) {}
    pub fn solution(&self) {}
    pub fn edit(&self) {}
    pub fn export(&self) {}
    pub fn import(&self) {}
    pub fn client_info(&self) {}
    pub fn submit_extra_source(&self) {}
    pub fn get_extra_sources(&self) {}
    pub fn remove_extra_source(&self) {}
}
