use std::collections::HashMap;
use std::io::{Error, Write, Read};
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
        data_map: HashMap<String, String>,
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
        for (k, v) in data_map.iter() {

            // transform String's into byte slice
            let kb = k.as_bytes();
            let vb = v.as_bytes();

            // add key len bytes
            for x in kb.len().to_be_bytes().iter() {
                buf.push(*x);
            }

            // add key
            for x in kb.into_iter() {
                buf.push(*x);
            }

            // add value len bytes
            for x in vb.len().to_be_bytes().iter() {
                buf.push(*x);
            }

            // add value
            for x in vb.into_iter() {
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

        let mut data: HashMap<String, String> = HashMap::new();

        let key = String::from("key_01_test");
        let val = String::from("val_01_test");

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
