use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, Error};
use std::convert::TryInto;

mod storage;

#[derive(Debug)]
pub struct Server {
    addr: String,
    client: Client,
    store: storage::Storage,
}

// Server handles TCP connections
impl Server {

    // new creates a new tcp server
    pub fn new(a: String) -> Server {
        Server{
            addr: a,
            client: Client::new(),
            store: storage::Storage::new(),
        }
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
    if tag_buf[0] != 99 {
        println!("invalid tag, exiting stream");
        return
    };

    // buffer to read data size in bytes
    let mut len_buf = [0; 4];

    // read data length from tcp stream
    let _n = match stream.read(&mut len_buf[..]) {
        Ok(n) => println!("{} bytes read from tcp stream", n),
        Err(e) => println!("error reading from stream: {}", e),
    };

    // convert bytes to u32
    let data_len = u32::from_be_bytes(len_buf);

    // buffer to read data of length given by len_buf
    let mut data_buf = Vec::with_capacity(data_len.try_into().unwrap());

    // read data length from tcp stream
    let _n = match stream.read(&mut data_buf[..]) {
        Ok(n) => println!("{} bytes read from tcp stream", n),
        Err(e) => println!("error reading from stream: {}", e),
    };

    // iterate over buffer
    for (i, x) in data_buf.into_iter().enumerate() {
        println!("data byte: {}", x)
    }
}

#[derive(Debug)]
pub struct Client {}

impl Client {

    pub fn new() -> Client {
        Client{}
    }

    pub fn send(self: &Self, data: String, addr: String) -> Result<(), Error> {

        // connect to server
        let mut stream = match TcpStream::connect(addr) {
            Ok(stream) => stream,
            Err(e) => return Err(e)
        };

        // create buffer
        let mut buf: Vec<u8> = Vec::new();

        // form data
        buf.push(b"S"[0]);
        buf.push(b"E"[0]);
        for (i, x) in data.as_bytes().into_iter().enumerate() {
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
    fn server_new() {

        let addr = "127.0.0.1:6000";
        let _s = Server::new(addr.to_string());
    }

    #[test]
    fn client_new() {
        Client::new();
    }
}