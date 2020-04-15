use std::net::TcpStream;
use std::collections::HashMap;
use std::io::{Read, Error};

// read_tag will read and verify the first byte in the stream
// expected tag hardcoded to b"0"
pub fn read_tag(mut stream: TcpStream) -> bool {

    // buffer to read tag
    let mut tag_buf = [0; 1];

    // read tag from tcp stream
    let _n = match stream.read(&mut tag_buf[..]) {
        Ok(n) => println!("{} bytes read from tcp stream", n),
        Err(e) => return false,
    };

    // match tag
    // 48 == b"0"
    if tag_buf[0] != 48 {
        println!("invalid tag {}", tag_buf[0]);
        return false
    };

    return true
}

//fn parse_map(data_buf: Vec<u8>) -> HashMap<String, String> {
//    let h = HashMap::new();
//
//    for d in &data_buf {
//
//    }
//
//    // iterate over bytes
//    // i == 0 is start of key 1
//    // hit record separator
//    // next is start of value 1
//    // hit record separator
//    // next is start of key 2...
//}

mod tests {

}