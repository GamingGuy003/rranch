use std::{io::Write, net::TcpStream};

use log::trace;

pub struct Client {
    pub socket: TcpStream,
}

impl Client {
    pub fn new(addr: &str, port: u16) -> Result<Self, std::io::Error> {
        Ok(Self {
            socket: TcpStream::connect(format!("{}:{}", addr, port))?,
        })
    }

    pub fn write_read(&mut self, content: &str) -> Result<String, std::io::Error> {
        self.write(content)?;
        self.read()
    }

    pub fn write(&mut self, content: &str) -> Result<(), std::io::Error> {
        let len = content.bytes().len();
        let msg = format!("{len} {content}");
        trace!("Trying to write {msg} to socket...");
        self.socket.write_all(msg.as_bytes())
    }

    pub fn write_raw(&mut self, bytes: Vec<u8>) -> Result<(), std::io::Error> {
        trace!("Trying to write {} raw bytes to socket...", bytes.len());
        self.socket.write_all(&bytes)?;
        Ok(())
    }

    pub fn read(&mut self) -> Result<String, std::io::Error> {
        let len = self.get_len()?;
        trace!("Trying to read {len} bytes from socket...");
        let mut read = vec![0; len as usize];
        std::io::Read::read_exact(&mut self.socket, &mut read)?;
        let ret = String::from_utf8(read.into_iter().collect()).unwrap_or_default();
        trace!("Received message was: {}", ret);
        Ok(ret)
    }

    fn get_len(&mut self) -> Result<u64, std::io::Error> {
        let mut buffer = Vec::new();
        for byte in std::io::Read::bytes(std::io::Read::by_ref(&mut self.socket)) {
            let byte = byte?;
            if byte == b' ' {
                break;
            }
            buffer.push(byte);
        }

        match String::from_utf8(buffer).unwrap_or_default().parse::<u64>() {
            Ok(len) => Ok(len),
            Err(err) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("{}", err),
            )),
        }
    }

    pub fn shutdown(&mut self) -> Result<(), std::io::Error> {
        self.socket.shutdown(std::net::Shutdown::Both)
    }
}
