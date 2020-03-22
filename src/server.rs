use std::io::{Error, Read};
use std::net::{TcpListener, TcpStream};

#[derive(Debug)]
pub struct Server {
    addr: String,
}

// Server handles TCP connections
impl Server {
    // new creates a new tcp server
    pub fn new(a: String) -> Server {
        Server { addr: a }
    }

    // start binds listener to socket
    pub fn start(self: &Self) -> Result<(), Error> {
        // listen for tcp connections at addr
        let listener = TcpListener::bind(&self.addr).unwrap();

        // create streams from incoming connctions then handle
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => handle(stream),
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}

// handle will own and operate the tcp connection
fn handle(mut stream: TcpStream) {
    // buffer to read tag
    let mut tag_buf = [0; 1];

    // read tag from tcp stream
    let _n = match stream.read(&mut tag_buf[..]) {
        Ok(n) => println!("{} bytes read from tcp stream", n),
        Err(e) => println!("error reading from stream: {}", e),
    };

    // match tag
    // 48 == b"0"
    if tag_buf[0] != 48 {
        println!("invalid tag {}", tag_buf[0]);
        return;
    };

    // buffer to read data size in bytes
    let mut len_buf = [0; 8];

    // read data length from tcp stream
    let _n = match stream.read(&mut len_buf[..]) {
        Ok(n) => println!("{} bytes read from tcp stream", n),
        Err(e) => println!("error reading from stream: {}", e),
    };

    // convert bytes to u32
    let data_len = usize::from_be_bytes(len_buf);

    // buffer to read data of length given by len_buf
    let mut data_buf = vec![0; data_len];

    // read data length from tcp stream
    let _n = match stream.read(&mut data_buf) {
        Ok(n) => println!("{} bytes read from tcp stream", n),
        Err(e) => println!("error reading from stream: {}", e),
    };

    // TODO: send to storage here instead of printing

    // iterate over buffer
    for (_i, x) in data_buf.into_iter().enumerate() {
        println!("data byte: {}", x)
    }
}

mod tests {

    use super::*;

    #[test]
    fn server_new() {
        let addr = "127.0.0.1:6000";
        let _s = Server::new(addr.to_string());
    }
}
