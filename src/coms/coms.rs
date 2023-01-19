use std::{
    io::{Read, Write},
    net::TcpStream,
};

use log::{debug, trace};

fn read(mut socket: &TcpStream, len: usize) -> Result<String, std::io::Error> {
    debug!("Trying to read {} bytes from socket...", len);
    let mut read = vec![0; len];
    let ret = match socket.read_exact(&mut read) {
        Ok(_) => String::from_utf8(read.into_iter().collect()).unwrap_or("".to_owned()),
        Err(err) => return Err(err),
    };
    trace!("Received message was: {:?}", ret);
    Ok(ret)
}

fn write(mut socket: &TcpStream, content: String) -> Result<(), std::io::Error> {
    debug!("Trying to write to socket...");
    trace!("Message is: {}", content);
    match socket.write(format!("{} {}", content.as_bytes().len(), content).as_bytes()) {
        Ok(_) => return Ok(()),
        Err(err) => return Err(err),
    }
}

pub fn get_len(socket: &TcpStream) -> Result<i32, std::io::Error> {
    debug!("Trying to fetch message length...");
    let mut buffer = Vec::new();
    for byte in socket.bytes() {
        let byte = byte?;
        if byte == b' ' {
            break;
        }
        buffer.push(byte);
    }
    let data = String::from_utf8(buffer).unwrap_or("".to_owned());
    trace!("Received length was: {}", data);
    match data.parse::<i32>() {
        Ok(len) => Ok(len),
        Err(err) => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("{}", err),
        )),
    }
}

//util function that writes message to socket and returns response or error
pub fn write_and_read(socket: &TcpStream, content: String) -> Result<String, std::io::Error> {
    write(&socket, content)?;
    read(&socket, get_len(&socket)? as usize)
}
