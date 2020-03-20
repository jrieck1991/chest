mod server;

fn main() {

    let s = server::Server::new(String::from("127.0.0.1:6000"));

    let s = match s.start() {
        Ok(()) => println!("server started successfully"),
        Err(e) => panic!("server.start err: {}", e),
    };
}
