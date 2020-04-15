use std::io::{Error, Read};
use std::net::{TcpListener, TcpStream};

mod transform;

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
fn handle(stream: TcpStream) {
    let h = transform::read_stream(&stream);
    println!("got map {:?}", h);
}

mod tests {

    use super::*;

    #[test]
    fn server_new() {
        let addr = "127.0.0.1:6000";
        let _s = Server::new(addr.to_string());
    }
}
