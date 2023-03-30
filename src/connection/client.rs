use std::{
    io::{self, Read, Write},
    net::TcpStream,
    process::exit,
};

use log::{debug, error, info, trace};

use super::info;

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
        debug!("Successfully shut down connection");
        exit(code)
    }

    pub fn auth(&mut self) -> Result<(), std::io::Error> {
        debug!("Trying to authenticate...");

        if self.authkey.is_none() {
            return Ok(());
        }

        let resp = self.write_and_read(&format!(
            "AUTH {}",
            self.authkey.clone().unwrap_or_default()
        ))?;

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

        let resp = self.write_and_read(&format!("SET_MACHINE_TYPE {}", self.client_type))?;

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

        let resp = self.write_and_read(&format!("SET_MACHINE_NAME {}", self.client_name))?;

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
        self.socket.read_exact(&mut read)?;
        let ret = String::from_utf8(read.into_iter().collect()).unwrap_or_default();
        trace!("Received message was: {:?}", ret);
        Ok(ret)
    }

    fn write(&mut self, content: &str) -> Result<(), std::io::Error> {
        trace!("Trying to write message to socket: {content}");

        self.socket
            .write(format!("{} {}", content.as_bytes().len(), content).as_bytes())?;
        Ok(())
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

    pub fn write_and_read(&mut self, content: &str) -> Result<String, std::io::Error> {
        self.write(content)?;
        self.read()
    }
}
