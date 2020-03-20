use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, Error};
use bytes::{BytesMut, BufMut};

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

    // continually loop and read from tcp stream
    loop {

        // init 1024 byte buffer
        let mut buf = BytesMut::with_capacity(1024);

        // read from tcp stream
        let _n = match stream.read(&mut buf) {
            Ok(n) => println!("{} bytes read from tcp stream", n),
            Err(e) => println!("error reading from stream: {}", e),
        };

        // iterate over buffer
        for (i, x) in buf.into_iter().enumerate() {

            // match starting byte
            if i == 0 {

                // continue onto payload
                if x == b"S"[0] {
                    println!("START hit, beginning");
                    continue
                }

                // malformed request
                break
            }

            // match ending byte
            // exit buffer
            if x == b"E"[0] {
                println!("EOF Hit, exiting");
                break
            }
        }
    }
}

#[derive(Debug)]
struct Client {}

impl Client {

    pub fn new() -> Client {
        Client{}
    }

    pub fn send(payload: String, addr: String) -> Result<(), Error> {

        // connect to server
        let mut stream = match TcpStream::connect(addr) {
            Ok(stream) => stream,
            Err(e) => return Err(e)
        };

        // create buffer
        let mut buf = BytesMut::with_capacity(1024);

        // form payload
        buf.put(&b"magic-byte"[..]);
        buf.put(&payload.as_bytes()[..]);
        buf.put(&b"eof"[..]);

        // write buffer to tcp connection
        let _n = match stream.write(&buf) {
            Ok(n) => n,
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