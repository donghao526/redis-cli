use std::net::TcpStream;
use std::io::Error;

pub struct Server {
    addr: String,
    port: u32,
}

impl Server {
    pub fn new(ip : String, port : u32) -> Server {
        Server{
            addr: ip,
            port: port,
        }
    }

    pub fn connect(&self) -> Result<TcpStream, Error> {
        let addr = format!("{}:{}", self.addr, self.port);
        TcpStream::connect(&*addr)
    }
}