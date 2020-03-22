use std::io::{Error, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Client {}

impl Client {
    pub fn new() -> Client {
        Client {}
    }

    pub fn send(self: &Self, data: String, addr: String) -> Result<(), Error> {
        // connect to server
        let mut stream = match TcpStream::connect(addr) {
            Ok(stream) => stream,
            Err(e) => return Err(e),
        };

        // create buffer
        let mut buf: Vec<u8> = Vec::new();

        // tag == u32 48
        buf.push(b"0"[0]);

        // data length
        let data_len = data.as_bytes().len();

        // add data length bytes
        for (_i, x) in data_len.to_be_bytes().iter().enumerate() {
            buf.push(*x);
        }

        // add data
        for (_i, x) in data.as_bytes().into_iter().enumerate() {
            buf.push(*x);
        }

        // write buffer to tcp connection
        let _n = match stream.write(&buf[..]) {
            Ok(n) => println!("{} bytes written to tcp stream", n),
            Err(e) => return Err(e),
        };

        Ok(())
    }
}

mod tests {

    use super::*;

    #[test]
    fn client_new() {
        Client::new();
    }
}
