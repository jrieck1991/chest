use std::collections::HashMap;
use std::io::{Error, Write};
use std::net::TcpStream;
use std::{thread, time};

#[derive(Debug)]
pub struct Client {}

impl Client {
    pub fn new() -> Client {
        Client {}
    }

    pub fn send(
        self: &Self,
        addr: String,
        data_map: HashMap<Vec<u8>, Vec<u8>>,
    ) -> Result<(), Error> {
        // connect to server
        let mut stream = match TcpStream::connect(addr) {
            Ok(stream) => stream,
            Err(e) => return Err(e),
        };

        // create buffer
        let mut buf: Vec<u8> = Vec::new();

        // tag == u32 48
        buf.push(b"0"[0]);

        // iterate over map to fill buffer
        for (k, _v) in data_map.iter() {
            // add key len bytes
            for (_i, x) in k.len().to_be_bytes().iter().enumerate() {
                buf.push(*x);
            }

            // add key
            for (_i, x) in k.into_iter().enumerate() {
                buf.push(*x);
            }

            // add key len bytes
            for (_i, x) in k.len().to_be_bytes().iter().enumerate() {
                buf.push(*x);
            }

            // add key
            for (_i, x) in k.into_iter().enumerate() {
                buf.push(*x);
            }
        }

        // write buffer to tcp connection
        let _n = match stream.write(&buf[..]) {
            Ok(n) => println!("{} bytes written to tcp stream", n),
            Err(e) => return Err(e),
        };

        Ok(())
    }
}

fn main() {
    let c = Client::new();

    loop {

        let mut data: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();

        let key: Vec<u8> = vec![123, 142, 123, 1];
        let val: Vec<u8> = vec![9, b"A"[0], 23, 6];

        data.insert(key, val);

        let _res = match c.send(String::from("localhost:6000"), data) {
            Ok(_res) => println!("send successful"),
            Err(_e) => break,
        };

        thread::sleep(time::Duration::from_millis(1000));
    }
}

mod tests {

   use super::*; 

    #[test]
    fn client_new() {
        Client::new();
    }
}
