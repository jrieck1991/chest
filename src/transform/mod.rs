use std::net::TcpStream;
use std::collections::HashMap;
use std::io::Error;

pub struct Transform {
    tag: u8,
}

impl Transform {

    pub fn new(t: u8) -> Transform {
        Transform{
            tag: t,
        }
    }

    pub fn encode_map(self &Self, HashMap<String, String>) -> Vec<u8> {

        let data_len = data.as_bytes().len();

        // create buffer
        let mut buf: Vec<u8> = Vec::with_capacity(data_len)

        // tag
        buf.push(self.tag);

        // add data length bytes
        for l in data_len.to_be_bytes().iter() {
            buf.push(*l);
        }

        // add data
        for d in data.to_be_bytes().iter() {
            buf.push(*d);
        }

        return buf;
    }

    pub fn decode_map(self: &Self, mut stream: TcpStream) -> HashMap<String, String> {

        // buffer to read tag
        let mut tag_buf = [0; 1];

        // read tag from tcp stream
        let _n = match stream.read(&mut tag_buf[..]) {
            Ok(n) => println!("{} bytes read from tcp stream", n),
            Err(e) => println!("error reading from stream: {}", e),
        };

        // match tag
        // 48 == b"0"
        if tag_buf[0] != self.tag {
            println!("invalid tag {}", tag_buf[0]);
            // TODO: return error here
            return;
        };

        // buffer to read data size in bytes
        let mut len_buf = [0; 8];

        // read data length from tcp stream
        let _n = match stream.read(&mut len_buf[..]) {
            Ok(n) => println!("{} bytes read from tcp stream", n),
            Err(e) => println!("error reading from stream: {}", e),
        };

        // convert bytes to usize
        let data_len = usize::from_be_bytes(len_buf);

        // buffer to read data of length given by len_buf
        let mut data_buf = vec![0; data_len];

        // read data length from tcp stream
        let _n = match stream.read(&mut data_buf) {
            Ok(n) => println!("{} bytes read from tcp stream", n),
            Err(e) => println!("error reading from stream: {}", e),
        };
    }
}

fn parse_map(data_buf: Vec<u8>) -> HashMap<String, String> {
    let h = HashMap::new();

    for d in &data_buf {

    }

    // iterate over bytes
    // i == 0 is start of key 1
    // hit record separator
    // next is start of value 1
    // hit record separator
    // next is start of key 2...
}

mod tests {

}