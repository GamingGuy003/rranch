use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

use log::{error, trace};

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

    pub fn close_connection(&self) -> Result<(), io::Error> {
        self.socket.shutdown(std::net::Shutdown::Both)
    }

    pub fn auth(&mut self) -> Result<(), std::io::Error> {
        if self.authkey.is_some() {
            let resp = match self
                .write_and_read(format!("AUTH {}", self.authkey.clone().unwrap_or_default()))
            {
                Ok(resp) => resp,
                Err(err) => return Err(err),
            };
        } else {
            return Ok(());
        }
        Ok(())
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
}
