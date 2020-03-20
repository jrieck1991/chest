mod server;

fn main() {

    let c = server::Client::new();

    let result = match c.send(String::from("sdkoghjsdlgkjds;lfajsfajsglksdjhglskghs"), String::from("127.0.0.1:6000")) {
        Ok(()) => println!("data sent successfully"),
        Err(e) => panic!("client.send err: {}", e),
    };
}
