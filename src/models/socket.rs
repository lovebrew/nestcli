use std::{
    io::{Read, Result},
    net::{Ipv4Addr, Shutdown, TcpStream},
};

pub struct Socket {
    stream: TcpStream,
    buffer: Vec<u8>,
}

const SOCKET_BUFFER_SIZE: usize = 0x1000;

impl Drop for Socket {
    fn drop(&mut self) {
        if let Err(result) = self.stream.shutdown(Shutdown::Both) {
            eprintln!("Failed to shutdown socket: {result}");
        }
    }
}

impl Socket {
    pub fn new(address: (Ipv4Addr, u16)) -> Result<Self> {
        let stream = TcpStream::connect(address)?;
        let buffer = vec![0; SOCKET_BUFFER_SIZE];
        Ok(Self { stream, buffer })
    }

    pub fn read(&mut self) -> Result<Option<&[u8]>> {
        match self.stream.read(&mut self.buffer) {
            Ok(0) => Ok(None),
            Ok(n) => Ok(Some(&self.buffer[..n])),
            Err(e) => Err(e),
        }
    }
}
